use crate::char_stream::CharStream;
use crate::common_token_factory::TokenFactory;
use crate::token::Token;

pub trait TokenSource {
    fn next_token(&mut self) -> Box<dyn Token>;
    fn get_line(&self) -> isize;
    fn get_char_position_in_line(&self) -> isize;
    fn get_input_stream(&mut self) -> &mut dyn CharStream;
    fn get_source_name(&self) -> String {
//        let input = self.get_input_stream();
//        input.get_source_name()
        unimplemented!()
    }
    //    fn set_token_factory<'c: 'b>(&mut self, f: &'c TokenFactory);
    fn get_token_factory(&self) -> &dyn TokenFactory;
}

// allows user to call parser with &mut reference to Lexer
impl<T> TokenSource for &mut T where T: TokenSource {
    fn next_token(&mut self) -> Box<dyn Token> {
        (**self).next_token()
    }

    fn get_line(&self) -> isize {
        (**self).get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        (**self).get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> &mut dyn CharStream {
        (**self).get_input_stream()
    }

    fn get_token_factory(&self) -> &dyn TokenFactory {
        (**self).get_token_factory()
    }
}

