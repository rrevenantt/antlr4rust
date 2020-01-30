use std::fmt::{Debug, Display};
use std::fmt::Formatter;

use crate::char_stream::CharStream;
use crate::int_stream::EOF;
use crate::token_source::TokenSource;

pub const TOKEN_INVALID_TYPE: isize = 0;
pub const TOKEN_EPSILON: isize = -2;
pub const TOKEN_MIN_USER_TOKEN_TYPE: isize = 1;
pub const TOKEN_EOF: isize = -1;
pub const TOKEN_DEFAULT_CHANNEL: isize = 0;
pub const TOKEN_HIDDEN_CHANNEL: isize = 1;
pub const HIDDEN: isize = TOKEN_HIDDEN_CHANNEL;


pub trait Token: Debug {
    fn get_source(&self) -> Option<(Box<dyn TokenSource>, Box<dyn CharStream>)>;
    fn get_token_type(&self) -> isize;
    fn get_channel(&self) -> isize;
    fn get_start(&self) -> isize;
    fn get_stop(&self) -> isize;
    fn get_line(&self) -> isize;
    fn get_column(&self) -> isize;

    fn get_text(&self) -> &str;
    fn set_text(&self, text: String);

    fn get_token_index(&self) -> isize;
    fn set_token_index(&mut self, v: isize);

    fn get_token_source(&self) -> &dyn TokenSource;
    fn get_input_stream(&self) -> &dyn CharStream;

    fn to_owned(&self) -> OwningToken;
}

#[derive(Debug, Clone)]
pub struct OwningToken {
    //    source: Option<(Box<TokenSource>,Box<CharStream>)>,
    pub token_type: isize,
    pub channel: isize,
    pub start: isize,
    pub stop: isize,
    pub token_index: isize,
    pub line: isize,
    pub column: isize,
    pub text: String,
    pub read_only: bool,
}

impl Display for OwningToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let txt = if self.token_type == TOKEN_EOF { "<EOF>" } else { &self.text };
        let txt = txt.replace("\n", "\\n");
        let txt = txt.replace("\r", "\\r");
        let txt = txt.replace("\t", "\\t");
//        let txt = escape_whitespaces(txt,false);
        f.write_fmt(format_args!("[@{},{}:{}='{}',<{}>{},{}:{}]",
                                 self.token_index,
                                 self.start,
                                 self.stop,
                                 txt,
                                 self.token_type,
                                 if self.channel > 0 { self.channel.to_string() } else { String::new() },
                                 self.line,
                                 self.column
        ))
    }
}


impl Token for OwningToken {
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

    fn get_source(&self) -> Option<(Box<dyn TokenSource>, Box<dyn CharStream>)> {
        unimplemented!()
    }

    fn get_token_index(&self) -> isize {
        self.token_index
    }

    fn set_token_index(&mut self, _v: isize) {
        self.token_index = _v
    }

    fn get_token_source(&self) -> &dyn TokenSource {
        unimplemented!()
    }

    fn get_input_stream(&self) -> &dyn CharStream {
        unimplemented!()
    }

    fn get_text(&self) -> &str {
        if self.token_type == EOF {
            "<EOF>"
        } else {
            &self.text
        }
    }

    fn set_text(&self, _text: String) {
        unimplemented!()
    }

    fn to_owned(&self) -> OwningToken {
        self.clone()
    }
}

pub struct CommonToken {
    base: OwningToken,
}

impl CommonToken {
    fn new_common_token(
        _source: Option<(Box<dyn TokenSource>, Box<dyn CharStream>)>,
        _token_type: isize,
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
