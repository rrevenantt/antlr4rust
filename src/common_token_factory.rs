use crate::token_source::TokenSource;
use crate::char_stream::CharStream;
use crate::token::Token;

use crate::token::BaseToken;

lazy_static! {
    pub static ref CommonTokenFactoryDEFAULT: Box<TokenFactory> =
        Box::new(CommonTokenFactory::new(false));
}

pub trait TokenFactory: Sync {
    fn create(
        &self,
        source: Option<(Box<TokenSource>, Box<CharStream>)>,
        ttype: isize,
        text: String,
        channel: isize,
        start: isize,
        stop: isize,
        line: isize,
        column: isize,
    ) -> Box<Token>;
}

pub struct CommonTokenFactory {
    copy_text: bool,
}

impl TokenFactory for CommonTokenFactory {
    fn create(
        &self,
        _source: Option<(Box<TokenSource>, Box<CharStream>)>,
        ttype: isize,
        text: String,
        channel: isize,
        start: isize,
        stop: isize,
        line: isize,
        column: isize,
    ) -> Box<Token> {
        Box::new(BaseToken {
            token_type: ttype,
            channel: channel,
            start: start,
            stop: stop,
            token_index: -1,
            line: line,
            column: column,
            text: text,
            readOnly: false,
        })
    }
}

impl CommonTokenFactory {
    pub fn new(copyText: bool) -> CommonTokenFactory {
        CommonTokenFactory {
            copy_text: copyText,
        }
    }

    fn create_thin(&self, _ttype: isize, _text: String) -> Box<Token> {
        unimplemented!()
    }
}
