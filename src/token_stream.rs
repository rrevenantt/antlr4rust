use std::borrow::{Borrow, BorrowMut};
use std::cmp::min;
use std::marker::{PhantomData, Unsize};
use std::ops::Deref;
use std::ptr::drop_in_place;

use crate::char_stream::InputData;
use crate::errors::ANTLRError;
use crate::int_stream::{IntStream, IterWrapper};
use crate::token::{OwningToken, Token, TOKEN_EOF, TOKEN_INVALID_TYPE};
use crate::token_factory::{CommonTokenFactory, TokenFactory};
use crate::token_source::TokenSource;
use better_any::{Tid, TidAble};

/// An `IntSteam` of `Token`s
///
/// Used as an input for `Parser`s
/// If there is an existing source of tokens, you should implement
/// `TokenSource`, not `TokenStream`
pub trait TokenStream<'input>: IntStream {
    /// Output token type
    type TF: TokenFactory<'input> + 'input;
    fn lt(&mut self, k: isize) -> Option<&<Self::TF as TokenFactory<'input>>::Tok>;
    fn get(&self, index: isize) -> &<Self::TF as TokenFactory<'input>>::Tok;
    fn get_inner(&self, index: isize) -> &<Self::TF as TokenFactory<'input>>::Inner;
    fn get_token_source(&self) -> &dyn TokenSource<'input, TF = Self::TF>;
    //    fn set_token_source(&self,source: Box<TokenSource>);
    fn get_all_text(&self) -> String;
    fn get_text_from_interval(&self, start: isize, stop: isize) -> String;
    //    fn get_text_from_rule_context(&self,context: RuleContext) -> String;
    fn get_text_from_tokens<T: Token + ?Sized>(&self, a: &T, b: &T) -> String
    where
        Self: Sized,
    {
        self.get_text_from_interval(a.get_token_index(), b.get_token_index())
    }
}

//
pub struct TokenIter<'a, 'input: 'a, T: TokenStream<'input>>(
    &'a mut T,
    bool,
    PhantomData<fn() -> &'input str>,
);

impl<'a, 'input: 'a, T: TokenStream<'input>> Iterator for TokenIter<'a, 'input, T> {
    type Item = OwningToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 {
            return None;
        }
        let result = self.0.lt(1).unwrap().borrow().to_owned();
        self.0.consume();
        if result.get_token_type() == TOKEN_EOF {
            self.1 = true;
        }
        Some(result)
    }
}

#[derive(Tid)]
pub struct UnbufferedTokenStream<'input, T: TokenSource<'input>> {
    token_source: T,
    pub(crate) tokens: Vec<<T::TF as TokenFactory<'input>>::Tok>,
    //todo prev token for lt(-1)
    pub(crate) current_token_index: isize,
    markers_count: isize,
    pub(crate) p: isize,
}

impl<'input, T: TokenSource<'input>> UnbufferedTokenStream<'input, T> {
    pub fn iter(&mut self) -> IterWrapper<'_, Self> { IterWrapper(self) }

    pub fn token_iter(&mut self) -> TokenIter<'_, 'input, Self> {
        TokenIter(self, false, PhantomData)
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

    fn get_buffer_start_index(&self) -> isize { self.current_token_index - self.p }

    pub(crate) fn fill(&mut self, need: isize) -> isize {
        for i in 0..need {
            if self.tokens.len() > 0
                && self.tokens.last().unwrap().borrow().get_token_type() == TOKEN_EOF
            {
                return i;
            }
            let mut token = self.token_source.next_token();
            token
                .borrow()
                .set_token_index(self.get_buffer_start_index() + self.tokens.len() as isize);
            self.tokens.push(token);
        }

        need
    }
}

impl<'input, T: TokenSource<'input>> TokenStream<'input> for UnbufferedTokenStream<'input, T> {
    type TF = T::TF;

    fn lt(&mut self, i: isize) -> Option<&<Self::TF as TokenFactory<'input>>::Tok> {
        if i == -1 {
            return self.tokens.get(self.p as usize - 1);
        }

        self.sync(i);

        self.tokens.get((self.p + i - 1) as usize)
    }

    fn get(&self, index: isize) -> &<Self::TF as TokenFactory<'input>>::Tok {
        &self.tokens[(index - self.get_buffer_start_index()) as usize]
    }

    fn get_inner(&self, index: isize) -> &<Self::TF as TokenFactory<'input>>::Inner {
        self.tokens[(index - self.get_buffer_start_index()) as usize].borrow()
    }

    fn get_token_source(&self) -> &dyn TokenSource<'input, TF = Self::TF> { &self.token_source }

    fn get_all_text(&self) -> String { self.get_text_from_interval(0, self.size()) }

    fn get_text_from_interval(&self, start: isize, stop: isize) -> String {
        //        println!("get_text_from_interval {}..{}",start,stop);
        //        println!("all tokens {:?}",self.tokens.iter().map(|x|x.as_ref().to_owned()).collect::<Vec<OwningToken>>());

        let buffer_start_index = self.get_buffer_start_index();
        let buffer_stop_index = buffer_start_index + self.tokens.len() as isize - 1;
        if start < buffer_start_index || stop > buffer_stop_index {
            panic!(
                "interval {}..={} not in token buffer window: {}..{}",
                start, stop, buffer_start_index, buffer_stop_index
            );
        }

        let a = start - buffer_start_index;
        let b = stop - buffer_start_index;

        let mut buf = String::new();
        for i in a..(b + 1) {
            let t = self.tokens[i as usize].borrow();
            if t.get_token_type() == TOKEN_EOF {
                break;
            }
            buf.extend(t.get_text().to_display().chars());
        }

        return buf;
    }
}

impl<'input, T: TokenSource<'input>> IntStream for UnbufferedTokenStream<'input, T> {
    fn consume(&mut self) -> Result<(), ANTLRError> {
        if self.la(1) == TOKEN_EOF {
            return Err(ANTLRError::IllegalStateError(
                "cannot consume EOF".to_owned(),
            ));
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
        self.lt(i)
            .map(|t| t.borrow().get_token_type())
            .unwrap_or(TOKEN_INVALID_TYPE)
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
                self.tokens.drain(0..self.p as usize);
                //todo drain assembly is almost 2x longer than
                // unsafe manual copy but need to bench before using unsafe
                //let new_len = self.tokens.len() - self.p as usize;
                // unsafe {
                //     // drop first p elements
                //     for i in 0..(self.p as usize) {
                //         drop_in_place(&mut self.tokens[i]);
                //     }
                //     // move len-p elements to beginning
                //     std::intrinsics::copy(
                //         &self.tokens[self.p as usize],
                //         &mut self.tokens[0],
                //         new_len,
                //     );
                //     self.tokens.set_len(new_len);
                // }

                self.p = 0;
            }
        }
    }

    #[inline(always)]
    fn index(&self) -> isize { self.current_token_index }

    fn seek(&mut self, mut index: isize) {
        if self.current_token_index == index {
            return;
        }
        if index > self.current_token_index {
            self.sync(index - self.current_token_index);
            index = min(index, self.get_buffer_start_index() + self.size() + 1);
        }
        let i = index - self.get_buffer_start_index();
        if i < 0 || i >= self.tokens.len() as isize {
            panic!()
        }

        self.p = i;
        self.current_token_index = index;
    }

    #[inline(always)]
    fn size(&self) -> isize { self.tokens.len() as isize }

    fn get_source_name(&self) -> String { self.token_source.get_source_name() }
}
