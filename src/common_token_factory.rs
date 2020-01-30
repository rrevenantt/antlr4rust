use crate::char_stream::CharStream;
use crate::token::{OwningToken, TOKEN_INVALID_TYPE};
use crate::token::Token;

lazy_static! {
    pub static ref CommonTokenFactoryDEFAULT: Box<dyn TokenFactory> =
        Box::new(CommonTokenFactory::new(false));
    pub static ref INVALID_TOKEN:Box<OwningToken> = Box::new(CommonTokenFactoryDEFAULT.as_ref().create(None,TOKEN_INVALID_TYPE,0,-1,-1,-1,-1).to_owned());
}

pub trait TokenFactory: Sync {
    fn create(
        &self,
        source: Option<&mut dyn CharStream>,
        ttype: isize,
        channel: isize,
        start: isize,
        stop: isize,
        line: isize,
        column: isize,
    ) -> Box<dyn Token>;
}

pub struct CommonTokenFactory {
    copy_text: bool,
}

impl TokenFactory for CommonTokenFactory {
    fn create(
        &self,
        source: Option<&mut dyn CharStream>,
        ttype: isize,
        channel: isize,
        start: isize,
        stop: isize,
        line: isize,
        column: isize,
    ) -> Box<dyn Token> {

        Box::new(OwningToken {
            token_type: ttype,
            channel,
            start,
            stop,
            token_index: -1,
            line,
            column,
            text: source.map(|x| {
                if stop >= x.size() || start >= x.size() { "<EOF>".to_owned() } else { x.get_text(start, stop) }
            })
                .unwrap_or(String::new()),
            read_only: false,
        })
    }
}

impl CommonTokenFactory {
    pub fn new(copy_text: bool) -> CommonTokenFactory {
        CommonTokenFactory {
            copy_text: copy_text,
        }
    }

    fn create_thin(&self, _ttype: isize, _text: String) -> Box<dyn Token> {
        unimplemented!()
    }
}
