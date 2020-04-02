use std::borrow::Cow::{Borrowed, Owned};
use std::marker::Unsize;
use std::ops::CoerceUnsized;

use crate::char_stream::CharStream;
use crate::token::{CommonToken, OwningToken, TOKEN_INVALID_TYPE};
use crate::token::Token;

lazy_static! {
    pub static ref CommonTokenFactoryDEFAULT: Box<CommonTokenFactory> =
        Box::new(CommonTokenFactory::new());
    pub static ref INVALID_TOKEN:Box<OwningToken> = CommonTokenFactoryDEFAULT.as_ref().create(None,TOKEN_INVALID_TYPE,None,0,-1,-1,-1,-1).to_owned();
}

/// This is a trait for creating tokens
pub trait TokenFactory<'a>: Sync {
    /// type of tokens emitted by this factory
    type Tok: Token + ?Sized + Unsize<dyn Token + 'a> + 'a;
    fn create<'b: 'a>(&'a self,
                      source: Option<&mut dyn CharStream<'b>>,
                      ttype: isize,
                      text: Option<String>,
                      channel: isize,
                      start: isize,
                      stop: isize,
                      line: isize,
                      column: isize,
    ) -> Box<Self::Tok>;
}

#[derive(Default)]
pub struct CowTokenFactory;

impl<'a> TokenFactory<'a> for CowTokenFactory {
    type Tok = CommonToken<'a>;

    fn create<'b: 'a>(&'a self,
                      source: Option<&mut dyn CharStream<'b>>,
                      ttype: isize,
                      text: Option<String>,
                      channel: isize,
                      start: isize,
                      stop: isize,
                      line: isize,
                      column: isize,
    ) -> Box<Self::Tok> {
        let text = match (text, source) {
            (Some(t), _) => Owned(t),

            (None, Some(x)) => {
                let t = if stop >= x.size() || start >= x.size() { "<EOF>" } else { x.get_text(start, stop) };
                Borrowed(t)
            }
            _ => Borrowed("")
        };
        Box::new(CommonToken {
            token_type: ttype,
            channel,
            start,
            stop,
            token_index: -1,
            line,
            column,
            text,
            read_only: false,
        })
    }
}

#[derive(Default)]
pub struct CommonTokenFactory {}

impl<'a> TokenFactory<'a> for CommonTokenFactory {
    type Tok = OwningToken;

    fn create<'b: 'a>(&'a self,
                      source: Option<&mut dyn CharStream<'b>>,
                      ttype: isize,
                      text: Option<String>,
                      channel: isize,
                      start: isize,
                      stop: isize,
                      line: isize,
                      column: isize,
    ) -> Box<Self::Tok> {
        let text = match (text, source) {
            (Some(t), _) => t,

            (None, Some(x)) => {
                let t = if stop >= x.size() || start >= x.size() { "<EOF>" } else { x.get_text(start, stop) };
                t.to_owned()
            }
            _ => String::new()
        };
        Box::new(OwningToken {
            token_type: ttype,
            channel,
            start,
            stop,
            token_index: -1,
            line,
            column,
            text,
            read_only: false,
        })
    }
}

impl CommonTokenFactory {
    pub fn new() -> CommonTokenFactory {
        CommonTokenFactory {}
    }

    fn create_thin(&self, _ttype: isize, _text: String) -> Box<dyn Token> {
        unimplemented!()
    }
}
