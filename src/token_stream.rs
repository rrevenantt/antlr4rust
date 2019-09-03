use crate::int_stream::{IntStream, IterWrapper};
use crate::token::{Token, TOKEN_EOF};
use crate::token_source::TokenSource;
use crate::interval_set::Interval;
use crate::rule_context::RuleContext;
use crate::errors::ANTLRError;
use std::cmp::min;
use std::intrinsics::copy_nonoverlapping;
use std::marker::PhantomData;

pub trait TokenStream: IntStream {
    fn lt(&mut self, k: isize) -> &dyn Token;
    fn get(&self, index: isize) -> &dyn Token;
    fn get_token_source(&self) -> &dyn TokenSource;
    //    fn set_token_source(&self,source: Box<TokenSource>);
    fn get_all_text(&self) -> String;
    fn get_text_from_interval(&self, from: &Interval) -> String;
    //    fn get_text_from_rule_context(&self,context: RuleContext) -> String;
    fn get_text_from_tokens(&self, a: &Token, b: &Token) -> String;
}


pub struct UnbufferedTokenStream<T: TokenSource> {
    token_source: T,
    tokens: Vec<Box<dyn Token>>,
    current_token_index: isize,
    markers_count: isize,
    p: isize,
}

impl<T: TokenSource> UnbufferedTokenStream<T> {
    pub fn iter(&mut self) -> IterWrapper<Self> {
        IterWrapper(self)
    }

    pub fn new(source: T) -> Self {
        UnbufferedTokenStream {
            token_source: source,
            tokens: vec![],
            current_token_index: 0,
            markers_count: 0,
            p: 0,
        }
    }

    fn sync(&mut self, i: isize) {
        let need = (self.p + i - 1) - self.tokens.len() as isize + 1;
        if need > 0 {
            self.fill(need);
        }
    }

    fn fill(&mut self, need: isize) -> isize {
        for i in 0..need {
            if self.tokens.len() > 0 && self.tokens.last().unwrap().get_token_type() == TOKEN_EOF {
                return i;
            }
            self.tokens.push(self.token_source.next_token());
        }

        need
    }
}

impl<T: TokenSource> TokenStream for UnbufferedTokenStream<T> {
    fn lt(&mut self, i: isize) -> &dyn Token {
        self.sync(i);

        self.tokens[(self.p + i - 1) as usize].as_ref()
    }

    fn get(&self, index: isize) -> &dyn Token {
        self.tokens[(index - (self.current_token_index - self.p)) as usize].as_ref()
    }

    fn get_token_source(&self) -> &TokenSource {
        unimplemented!()
    }


//    fn set_token_source(&self, source: Box<TokenSource>) {
//        unimplemented!()
//    }

    fn get_all_text(&self) -> String {
        unimplemented!()
    }

    fn get_text_from_interval(&self, from: &Interval) -> String {
        unimplemented!()
    }

    fn get_text_from_tokens(&self, a: &Token, b: &Token) -> String {
        unimplemented!()
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
        self.lt(i).get_token_type()
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

    fn index(&self) -> isize {
        self.current_token_index
    }

    fn seek(&mut self, mut index: isize) {
        if self.current_token_index == index { return; }

        if index > self.current_token_index {
            self.sync(index - self.current_token_index);
            index = min(index, self.current_token_index - self.p);
        }
        let i = index - self.current_token_index - self.p;
        if i < 0 || i >= self.tokens.len() as isize { panic!() }

        self.p = i;
        self.current_token_index = index;
    }

    fn size(&self) -> isize {
        panic!();
    }

    fn get_source_name(&self) -> String {
        self.token_source.get_source_name()
    }
}