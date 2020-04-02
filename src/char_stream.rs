use crate::int_stream::IntStream;
use crate::interval_set::Interval;
use crate::token::Token;

pub trait CharStream<'a>: IntStream {
    fn get_text(&self, a: isize, b: isize) -> &'a str;
    fn get_text_from_tokens(&self, start: &dyn Token, end: &dyn Token) -> &'a str;
    fn get_text_from_interval(&self, interval: &Interval) -> &'a str;
}
