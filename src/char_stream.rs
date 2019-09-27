use crate::int_stream::IntStream;
use crate::interval_set::Interval;
use crate::token::Token;

pub trait CharStream: IntStream {
    fn get_text(&self, a: isize, b: isize) -> String;
    fn get_text_from_tokens(&self, start: &Token, end: &Token) -> &str;
    fn get_text_from_interval(&self, interval: &Interval) -> String;
}
