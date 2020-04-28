use std::borrow::{Borrow, BorrowMut, Cow};
use std::cell::Cell;
use std::convert::identity;
use std::fmt::{Debug, Display};
use std::fmt::Formatter;
use std::ops::{CoerceUnsized, Deref, DerefMut};
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

use crate::char_stream::CharStream;
use crate::int_stream::EOF;
use crate::token_factory::{INVALID_COMMON, INVALID_OWNING};
use crate::token_source::TokenSource;

pub const TOKEN_INVALID_TYPE: isize = 0;
pub const TOKEN_EPSILON: isize = -2;
pub const TOKEN_MIN_USER_TOKEN_TYPE: isize = 1;
pub const TOKEN_EOF: isize = -1;
pub const TOKEN_DEFAULT_CHANNEL: isize = 0;
pub const TOKEN_HIDDEN_CHANNEL: isize = 1;
pub const HIDDEN: isize = TOKEN_HIDDEN_CHANNEL;

/// Trait for custom token implementations
///
/// For proper parsing, implementations must have valid implementations
/// for at least token type and token index.
///
/// Other members are mostly required for error reporting
pub trait Token: Debug {
    // fn get_source(&self) -> Option<(Box<dyn TokenSource>, Box<dyn CharStream>)>;
    fn get_token_type(&self) -> isize;
    fn get_channel(&self) -> isize;
    fn get_start(&self) -> isize;
    fn get_stop(&self) -> isize;
    fn get_line(&self) -> isize;
    fn get_column(&self) -> isize;

    fn get_text(&self) -> &str;
    fn set_text(&self, text: String);

    fn get_token_index(&self) -> isize;
    fn set_token_index(&self, v: isize);

    // fn get_token_source(&self) -> &dyn TokenSource;
    // fn get_input_stream(&self) -> &dyn CharStream;

    fn to_owned(&self) -> OwningToken;

}

///automatically implemented interface for passing tokens behind different kinds of ownership
// pub trait TokenWrapper<'ref>: BorrowMut<<Self as TokenWrapper>::Inner>{
//     type Inner:Token + ?Sized + 'ref;
// }
//
// impl<'a,T:Token + ?Sized + 'a> TokenWrapper<'a> for Box<T> {
//     type Inner = T;
// }
//
// impl<'a,T:Token + ?Sized + 'a> TokenWrapper<'a> for Box<T> {
//     type Inner = T;
// }
//
// impl<'a,T:Token + ?Sized + 'a> TokenWrapper<'a> for &'a mut T {
//     type Inner = ;
// }

// impl<T:DerefMut + Debug + BorrowMut<<T as Deref>::Target>> Token for T where <T as Deref>::Target:Token{
//     fn get_token_type(&self) -> isize { self.deref().get_token_type() }
//
//     fn get_channel(&self) -> isize { self.deref().get_channel() }
//
//     fn get_start(&self) -> isize { self.deref().get_start() }
//
//     fn get_stop(&self) -> isize { self.deref().get_stop() }
//
//     fn get_line(&self) -> isize { self.deref().get_line() }
//
//     fn get_column(&self) -> isize { self.deref().get_column() }
//
//     fn get_text(&self) -> &str {self.deref().get_text()}
//
//     fn set_text(&self, text: String) {self.deref().set_text()}
//
//     fn get_token_index(&self) -> isize {self.deref().get_token_index()}
//
//     fn set_token_index(&mut self, v: isize) {self.deref().set_token_index()}
//
//     fn to_owned(&self) -> OwningToken {self.deref().to_owned()}
// }

pub type OwningToken = GenericToken<String>;
pub type CommonToken<'a> = GenericToken<Cow<'a, str>>;

#[derive(Debug)]
pub struct GenericToken<T: Borrow<str> + Debug = String> {
    //    source: Option<(Box<TokenSource>,Box<CharStream>)>,
    pub token_type: isize,
    pub channel: isize,
    pub start: isize,
    pub stop: isize,
    pub token_index: AtomicIsize,
    pub line: isize,
    pub column: isize,
    pub text: T,
    pub read_only: bool,
}

impl<T: Borrow<str> + Debug + Clone> Clone for GenericToken<T> {
    fn clone(&self) -> Self {
        Self {
            token_type: self.token_type,
            channel: self.channel,
            start: self.start,
            stop: self.stop,
            token_index: AtomicIsize::new(self.get_token_index()),
            line: self.line,
            column: self.column,
            text: self.text.clone(),
            read_only: false,
        }
    }
}

impl<T: Borrow<str> + Debug> Display for GenericToken<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let txt = if self.token_type == TOKEN_EOF { "<EOF>" } else { self.text.borrow() };
        let txt = txt.replace("\n", "\\n");
        let txt = txt.replace("\r", "\\r");
        let txt = txt.replace("\t", "\\t");
//        let txt = escape_whitespaces(txt,false);
        f.write_fmt(format_args!("[@{},{}:{}='{}',<{}>{},{}:{}]",
                                 self.get_token_index(),
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


// impl<T: Borrow<str> + Debug> TokenWrapper for GenericToken<T> { type Inner = Self; }

impl<T: Borrow<str> + Debug> Token for GenericToken<T> {
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

    // fn get_source(&self) -> Option<(Box<dyn TokenSource>, Box<dyn CharStream>)> {
    //     unimplemented!()
    // }

    fn get_token_index(&self) -> isize {
        self.token_index.load(Ordering::Relaxed)
    }

    fn set_token_index(&self, _v: isize) {
        self.token_index.store(_v, Ordering::Relaxed)
    }

    // fn get_token_source(&self) -> &dyn TokenSource {
    //     unimplemented!()
    // }
    //
    // fn get_input_stream(&self) -> &dyn CharStream {
    //     unimplemented!()
    // }

    fn get_text(&self) -> &str {
        if self.token_type == EOF {
            "<EOF>"
        } else {
            self.text.borrow()
        }
    }

    fn set_text(&self, _text: String) {
        unimplemented!()
    }

    fn to_owned(&self) -> OwningToken {
        OwningToken {
            token_type: self.token_type,
            channel: self.channel,
            start: self.start,
            stop: self.stop,
            token_index: AtomicIsize::new(self.get_token_index()),
            line: self.line,
            column: self.column,
            text: self.text.borrow().to_owned(),
            read_only: self.read_only,
        }
    }
}

impl Default for &'_ OwningToken {
    fn default() -> Self {
        &**INVALID_OWNING
    }
}

impl Default for &'_ CommonToken<'_> {
    fn default() -> Self {
        &**INVALID_COMMON
    }
}

//
// impl CommonToken {
//     fn new_common_token(
//         _source: Option<(Box<dyn TokenSource>, Box<dyn CharStream>)>,
//         _token_type: isize,
//         _channel: isize,
//         _start: isize,
//         _stop: isize,
//     ) -> CommonToken {
//         unimplemented!()
//     }
//
//     fn clone(&self) -> CommonToken {
//         unimplemented!()
//     }
// }
