use std::cmp::min;
use std::intrinsics::copy_nonoverlapping;
use std::ops::Deref;

use crate::errors::ANTLRError;
use crate::int_stream::{IntStream, IterWrapper};
use crate::token::{OwningToken, Token, TOKEN_EOF, TOKEN_INVALID_TYPE};
use crate::token_source::TokenSource;

pub trait TokenStream: IntStream {
    fn lt(&mut self, k: isize) -> Option<&dyn Token>;
    fn get(&self, index: isize) -> &dyn Token;
    fn get_token_source(&self) -> &dyn TokenSource;
    //    fn set_token_source(&self,source: Box<TokenSource>);
    fn get_all_text(&self) -> String;
    fn get_text_from_interval(&self, start: isize, stop: isize) -> String;
    //    fn get_text_from_rule_context(&self,context: RuleContext) -> String;
    fn get_text_from_tokens(&self, a: &dyn Token, b: &dyn Token) -> String;
}

pub struct TokenIter<'a, T: TokenStream>(&'a mut T, bool);

impl<'a, T: TokenStream> Iterator for TokenIter<'a, T> {
    type Item = OwningToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 { return None }
        let result = self.0.lt(1).unwrap().to_owned();
        self.0.consume();
        if result.get_token_type() == TOKEN_EOF { self.1 = true; }
        Some(result)
    }
}

//impl<T:TokenStream> IntoIterator for T{
//    type Item = OwningToken;
//    type IntoIter = TokenIter<'_,T>;
//
//    fn into_iter(self) -> Self::IntoIter {
//        unimplemented!()
//    }
//}

pub struct UnbufferedTokenStream<T: TokenSource> {
    token_source: T,
    pub(crate) tokens: Vec<Box<dyn Token>>,
    //todo prev token for lt(-1)
    pub(crate) current_token_index: isize,
    markers_count: isize,
    pub(crate) p: isize,

}

impl<T: TokenSource> UnbufferedTokenStream<T> {
    pub fn iter(&mut self) -> IterWrapper<Self> {
        IterWrapper(self)
    }

    pub fn token_iter(&mut self) -> TokenIter<Self> {
        TokenIter(self, false)
    }

    pub fn new_buffered(source: T) -> Self {
        let mut a = UnbufferedTokenStream::new_unbuffered(source);
        a.mark();
        a
    }


    pub fn new_unbuffered(source: T) -> Self {
        UnbufferedTokenStream {
            token_source: source,
            tokens: vec![],
            current_token_index: 0,
            markers_count: 0,
            p: 0,
        }
    }

    fn sync(&mut self, want: isize) {
        let need = (self.p + want - 1) - self.tokens.len() as isize + 1;
        if need > 0 {
            self.fill(need);
        }
    }

    fn get_buffer_start_index(&self) -> isize {
        self.current_token_index - self.p
    }

    pub(crate) fn fill(&mut self, need: isize) -> isize {
        for i in 0..need {
            if self.tokens.len() > 0 && self.tokens.last().unwrap().get_token_type() == TOKEN_EOF {
                return i;
            }
            let mut token = self.token_source.next_token();
            token.set_token_index(self.get_buffer_start_index() + self.tokens.len() as isize);
            self.tokens.push(token);
        }

        need
    }
}

impl<T: TokenSource> TokenStream for UnbufferedTokenStream<T> {
    fn lt(&mut self, i: isize) -> Option<&dyn Token> {
        if i == -1 {
            return self.tokens.get(self.p as usize - 1).map(Deref::deref)
        }

        self.sync(i);

        self.tokens.get((self.p + i - 1) as usize).map(Deref::deref)
    }

    fn get(&self, index: isize) -> &dyn Token {
        self.tokens[(index - self.get_buffer_start_index()) as usize].as_ref()
    }

    fn get_token_source(&self) -> &dyn TokenSource {
        &self.token_source
    }


//    fn set_token_source(&self, source: Box<TokenSource>) {
//        unimplemented!()
//    }

    fn get_all_text(&self) -> String {
        String::new()
    }

    fn get_text_from_interval(&self, start: isize, stop: isize) -> String {
//        println!("get_text_from_interval {}..{}",start,stop);
//        println!("all tokens {:?}",self.tokens.iter().map(|x|x.as_ref().to_owned()).collect::<Vec<OwningToken>>());

        let buffer_start_index = self.get_buffer_start_index();
        let buffer_stop_index = buffer_start_index + self.tokens.len() as isize - 1;
        if start < buffer_start_index || stop > buffer_stop_index {
            panic!("interval {}..={} not in token buffer window: {}..{}", start, stop, buffer_start_index, buffer_stop_index);
        }

        let a = start - buffer_start_index;
        let b = stop - buffer_start_index;

        let mut buf = String::new();
        for i in a..(b + 1) {
            let t = &self.tokens[i as usize];
            if t.get_token_type() == TOKEN_EOF { break }
            buf.extend(t.get_text().chars());
        }

        return buf;
    }

    fn get_text_from_tokens(&self, a: &dyn Token, b: &dyn Token) -> String {
        self.get_text_from_interval(a.get_token_index(), b.get_token_index())
    }
}

impl<T: TokenSource> IntStream for UnbufferedTokenStream<T> {
    fn consume(&mut self) -> Result<(), ANTLRError> {
        if self.la(1) == TOKEN_EOF {
            return Err(ANTLRError::IllegalStateError("cannot consume EOF".to_owned()));
        }

        if self.p == self.tokens.len() as isize && self.markers_count == 0 {
            self.tokens.clear();
            self.p = -1;
        }

        self.p += 1;
        self.current_token_index += 1;

        self.sync(1);
        Ok(())
    }

    fn la(&mut self, i: isize) -> isize {
        self.lt(i).map(Token::get_token_type).unwrap_or(TOKEN_INVALID_TYPE)
    }

    fn mark(&mut self) -> isize {
        self.markers_count += 1;
        -self.markers_count
    }

    fn release(&mut self, marker: isize) {
        assert_eq!(marker, -self.markers_count);

        self.markers_count -= 1;
        if self.markers_count == 0 {
            if self.p > 0 {
                unsafe {
                    copy_nonoverlapping(
                        &self.tokens[self.p as usize] as *const Box<dyn Token>,
                        &mut self.tokens[0] as *mut Box<dyn Token>,
                        self.tokens.len() - self.p as usize,
                    )
                }
            }
        }
    }

    #[inline(always)]
    fn index(&self) -> isize {
        self.current_token_index
    }

    fn seek(&mut self, mut index: isize) {
        if self.current_token_index == index { return; }
        if index > self.current_token_index {
            self.sync(index - self.current_token_index);
            index = min(index, self.get_buffer_start_index() + self.size() + 1);
        }
        let i = index - self.get_buffer_start_index();
        if i < 0 || i >= self.tokens.len() as isize { panic!() }

        self.p = i;
        self.current_token_index = index;
    }

    #[inline(always)]
    fn size(&self) -> isize {
        self.tokens.len() as isize
    }

    fn get_source_name(&self) -> String {
        self.token_source.get_source_name()
    }
}