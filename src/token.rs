use crate::token_source::TokenSource;
use crate::char_stream::CharStream;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

pub const token_invalid_type: isize = 0;
pub const TOKEN_EPSILON: isize = -2;
pub const token_min_user_token_type: isize = 1;
pub const TOKEN_EOF: isize = -1;
pub const token_default_channel: isize = 0;
pub const token_hidden_channel: isize = 1;

pub trait Token: Debug {
    fn get_source(&self) -> Option<(Box<TokenSource>, Box<CharStream>)>;
    fn get_token_type(&self) -> isize;
    fn get_channel(&self) -> isize;
    fn get_start(&self) -> isize;
    fn get_stop(&self) -> isize;
    fn get_line(&self) -> isize;
    fn get_column(&self) -> isize;

    fn get_text(&self) -> String;
    fn set_text(&self, text: String);

    fn get_token_index(&self) -> isize;
    fn set_token_index(&self, v: isize);

    fn get_token_source(&self) -> &TokenSource;
    fn get_input_stream(&self) -> &CharStream;
}

//impl Debug for Token {
//    fn fmt(&self, _f: &mut Formatter) -> Result {
//        unimplemented!()
//    }
//}
#[derive(Debug)]
pub struct BaseToken {
    //    source: Option<(Box<TokenSource>,Box<CharStream>)>,
    pub token_type: isize,
    pub channel: isize,
    pub start: isize,
    pub stop: isize,
    pub token_index: isize,
    pub line: isize,
    pub column: isize,
    pub text: String,
    pub readOnly: bool,
}

impl Token for BaseToken {
    fn get_channel(&self) -> isize {
        self.channel
    }

    fn get_start(&self) -> isize {
        self.start
    }

    fn get_stop(&self) -> isize {
        self.stop
    }

    fn get_line(&self) -> isize {
        self.line
    }

    fn get_column(&self) -> isize {
        self.column
    }

    fn get_token_type(&self) -> isize {
        self.token_type
    }

    fn get_source(&self) -> Option<(Box<TokenSource>, Box<CharStream>)> {
        unimplemented!()
    }

    fn get_token_index(&self) -> isize {
        self.token_index
    }

    fn set_token_index(&self, _v: isize) {
        unimplemented!()
    }

    fn get_token_source(&self) -> &TokenSource {
        unimplemented!()
    }

    fn get_input_stream(&self) -> &CharStream {
        unimplemented!()
    }

    fn get_text(&self) -> String {
        unimplemented!()
    }

    fn set_text(&self, _text: String) {
        unimplemented!()
    }
}

pub struct CommonToken {
    base: BaseToken,
}

impl CommonToken {
    fn new_common_token(
        _source: Option<(Box<TokenSource>, Box<CharStream>)>,
        _tokenType: isize,
        _channel: isize,
        _start: isize,
        _stop: isize,
    ) -> CommonToken {
        unimplemented!()
    }

    fn clone(&self) -> CommonToken {
        unimplemented!()
    }
}

//    fn String(&self) -> String { unimplemented!() }
