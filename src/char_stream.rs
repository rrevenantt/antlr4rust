use std::borrow::{Borrow, Cow};

use crate::int_stream::IntStream;
use crate::interval_set::Interval;
use crate::token::Token;

pub trait CharStream<'a>: IntStream {
    type T: 'a + Into<Cow<'a, str>> + Borrow<str>;
    fn get_text(&self, a: isize, b: isize) -> Self::T;
    fn get_text_from_tokens(&self, start: &dyn Token, end: &dyn Token) -> Self::T {
        self.get_text(start.get_token_index(), end.get_token_index())
    }
    fn get_text_from_interval(&self, i: &Interval) -> Self::T {
        self.get_text(i.a, i.b)
    }
}
