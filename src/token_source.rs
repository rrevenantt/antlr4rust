use crate::char_stream::CharStream;
use crate::token::Token;
use crate::common_token_factory::TokenFactory;

use std::rc::Rc;
use std::cell::RefCell;

pub trait TokenSource {
    fn next_token(&mut self) -> Box<Token>;
    fn skip(&mut self);
    fn more(&mut self);
    fn get_line(&self) -> isize;
    fn get_char_position_in_line(&self) -> isize;
    fn get_input_stream(&self) -> &RefCell<Box<CharStream>>;
    fn get_source_name(&self) -> String {
        let input = self.get_input_stream();
        let temp = input.borrow();
        temp.get_source_name()
    }
    //    fn set_token_factory<'c: 'b>(&mut self, f: &'c TokenFactory);
    fn get_token_factory(&self) -> &TokenFactory;
}
