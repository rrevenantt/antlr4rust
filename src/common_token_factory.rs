use crate::token_source::TokenSource;
use crate::char_stream::CharStream;
use crate::token::Token;

use crate::token::OwningToken;

lazy_static! {
    pub static ref CommonTokenFactoryDEFAULT: Box<TokenFactory> =
        Box::new(CommonTokenFactory::new(false));
}

pub trait TokenFactory: Sync {
    fn create(
        &self,
        source: Option<&mut CharStream>,
        ttype: isize,
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
        source: Option<&mut CharStream>,
        ttype: isize,
        channel: isize,
        start: isize,
        stop: isize,
        line: isize,
        column: isize,
    ) -> Box<Token> {
        Box::new(OwningToken {
            token_type: ttype,
            channel,
            start,
            stop,
            token_index: -1,
            line,
            column,
            text: source.map(|x| x.get_text(start, stop)).unwrap_or(String::new()),
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
