use crate::char_stream::CharStream;
//use lexer_atn_simulator::ILexerATNSimulator;
use crate::common_token_factory::TokenFactory;
use crate::error_listener::{ErrorListener, DefaultErrorListener};
use crate::errors::ANTLRError;
use crate::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use crate::recognizer::Recognizer;

use std::borrow::{Borrow, BorrowMut};

use crate::token::{Token, token_invalid_type};
use crate::token_source::TokenSource;
use std::cell::{RefCell, RefMut};

use std::rc::Rc;
use crate::int_stream::EOF;
use std::time::Duration;
use std::ops::{DerefMut, Deref};


pub trait Lexer: TokenSource {
    fn set_channel(&mut self, v: isize);

    fn push_mode(&mut self, m: isize);

    fn pop_mode(&mut self) -> Option<isize>;

    fn set_type(&mut self, t: isize);

    fn set_mode(&mut self, m: isize);

    fn more(&mut self);

    fn skip(&mut self);

    fn reset(&mut self);
}

pub struct BaseLexer<'b> {
    interpreter: Option<LexerATNSimulator>,
    input: Option<Box<dyn CharStream>>,
//    recog: Rc<RefCell<Box<dyn Recognizer>>>,

    factory: &'b TokenFactory,

    error_listeners: RefCell<Vec<Box<ErrorListener>>>,

    token_start_char_index: isize,
    pub token_start_line: isize,
    pub token_start_column: isize,
    token_type: isize,
    token: Option<Box<Token>>,
    hit_eof: bool,
    channel: isize,
    mode_stack: Vec<isize>,
    mode: isize,
    text: String,
}

//pub struct CoreLexer<'b>{
//    pub(crate) lexer:BaseLexer<'b>,
//    pub(crate) input:Box<dyn CharStream>,
//    pub(crate) recog:Box<dyn Recognizer>
//}


//pub(crate) struct LexerInternalContext<'a>{
//    lexer:&'a mut dyn Lexer,
//    input:&'a mut dyn CharStream,
////    recog:&'a mut dyn Recognizer
//}

pub const LEXER_DEFAULT_MODE: isize = 0;
pub const LEXER_MORE: isize = -2;
pub const LEXER_SKIP: isize = -3;

pub const lexer_default_token_channel: isize = super::token::TOKEN_DEFAULT_CHANNEL;
pub const lexer_hidden: isize = super::token::TOKEN_HIDDEN_CHANNEL;
pub const LEXER_MIN_CHAR_VALUE: isize = 0x0000;
pub const LEXER_MAX_CHAR_VALUE: isize = 0x10FFFF;

impl<'b> BaseLexer<'b> {
    pub fn get_interpreter(&self) -> Option<&LexerATNSimulator> { self.interpreter.as_ref() }

    fn safe_match(&self) {
        unimplemented!()
    }

//    fn set_input_stream(&mut self, input: Box<CharStream>) {
//        self.input = Some(RefCell::new(input));
//    }

    fn emit_token(&mut self, token: Box<Token>) {
        self.token = Some(token);
    }

    //    fn get_token_source_char_stream_pair(&self) -> * TokenSourceCharStreamPair { unimplemented!() }

    fn emit(&mut self) {
        let stop = self.get_char_index() - 1;
        let token = self.factory.create(
            Some(self.input.as_mut().unwrap().as_mut()),
            self.token_type,
            self.channel,
            self.token_start_char_index,
            stop,
            self.token_start_line,
            self.token_start_column,

        );
        self.emit_token(token);
    }

    fn emit_eof(&mut self) {
        let token = self.factory.create(
            None,
            super::int_stream::EOF,
            lexer_default_token_channel,
            self.get_char_index(),
            self.get_char_index() - 1,
            self.get_line(),
            self.get_char_position_in_line(),
        );
        self.emit_token(token)
    }

    fn get_type(&self) -> isize {
        self.token_type
    }

    fn get_char_index(&self) -> isize {
        self.input.as_ref().unwrap().index()
    }

    pub fn get_text(&self) -> String {
        self.input.as_ref().unwrap().get_text(self.token_start_char_index, self.get_char_index() - 1)
    }

    fn set_text(&self, _text: String) {
        unimplemented!()
    }

    fn get_all_tokens(&self) -> Vec<Box<Token>> {
        unimplemented!()
    }

    fn get_error_display_for_char(&self, _c: char) -> String {
        unimplemented!()
    }

    fn get_char_error_display(&self, _c: char) -> String {
        unimplemented!()
    }

    pub fn add_error_listener(&mut self, listener: Box<ErrorListener>) {
        self.error_listeners.borrow_mut().push(listener);
    }

    pub fn remove_error_listeners(&mut self) {
        self.error_listeners.borrow_mut().clear();
    }

    pub fn new_base_lexer(
        input: Box<dyn CharStream>,
        interpreter: LexerATNSimulator,
//        recog:Rc<RefCell<Box<dyn Recognizer>>>
    ) -> BaseLexer<'b> {
        BaseLexer {
            interpreter: Some(interpreter),
            input: Some(input),
//            recog,
            factory: super::common_token_factory::CommonTokenFactoryDEFAULT.as_ref(),
            error_listeners: RefCell::new(vec![Box::new(DefaultErrorListener {})]),
            token_start_char_index: 0,
            token_start_line: 0,
            token_start_column: 0,
            token_type: super::token::token_invalid_type,
            text: "".into(),
            token: None,
            hit_eof: false,
            channel: super::token::TOKEN_DEFAULT_CHANNEL,
            //            token_factory_source_pair: None,
            mode_stack: Vec::new(),
            mode: self::LEXER_DEFAULT_MODE,
        }
    }
}

impl<'b> TokenSource for BaseLexer<'b> {
    fn next_token(&mut self) -> Box<Token> {
        assert!(self.input.is_some());

        let _marker = self.input.as_mut().unwrap().mark();
        'outer: loop {
            if self.hit_eof {
                self.emit_eof();
                break;
            }
            self.token = None;
            self.channel = lexer_default_token_channel;
            self.token_start_column = self.interpreter.as_ref().unwrap().get_char_position_in_line();
            self.token_start_line = self.interpreter.as_ref().unwrap().get_line();
            self.text = String::new();
            let index = self.get_input_stream().index();
            self.token_start_char_index = index;

            'inner: loop {
                let ttype;
                self.token_type = token_invalid_type;
                {
                    // detach from self, to allow self to be passed deeper
                    let mut interpreter = self.interpreter.take().unwrap();
//                    let mut input = self.input.take().unwrap();
                    let result = interpreter
                        .match_token(self.mode, self);
                    self.interpreter = Some(interpreter);

                    ttype = match result {
                        Ok(ttype) => {
//                            println!("new mode {}",self.mode);
                            ttype
                        },
                        Err(err) => {
//                            println!("error, recovering");
                            notify_listeners(&mut self.error_listeners.borrow_mut(), &err, self);
                            self.interpreter.as_mut().unwrap().recover(err, self.input.as_mut().unwrap().deref_mut());
                            LEXER_SKIP
                        }
                    };
//                    self.input = Some(input)
                }
                if self.get_input_stream().la(1) == super::int_stream::EOF {
                    self.hit_eof = true;
                }

                if self.token_type == token_invalid_type {
                    self.token_type = ttype;
                }

                if self.token_type == LEXER_SKIP {
                    continue 'outer;
                }

                if self.token_type != LEXER_MORE {
                    break;
                }
            }

            if self.token.is_none() {
                self.emit();
                break;
            }
        }
        self.input.as_mut().unwrap().release(_marker);
        self.token.take().unwrap()
    }

    fn get_line(&self) -> isize {
        self.interpreter.as_ref().unwrap().get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.interpreter.as_ref().unwrap().get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> &mut dyn CharStream {
        self.input.as_mut().unwrap().deref_mut()
    }

//    fn set_token_factory<'c: 'b>(&mut self, f: &'c TokenFactory) {
//        self.factory = f;
//    }

    fn get_token_factory(&self) -> &TokenFactory {
        self.factory
    }
}

fn notify_listeners(_liseners: &mut Vec<Box<ErrorListener>>, e: &ANTLRError, lexer: &BaseLexer) {
    let text = format!("token recognition error at: '{}'", lexer.input.as_ref().unwrap().get_text(lexer.token_start_char_index, lexer.get_char_index()));
    for listener in _liseners.iter_mut() {
        listener.syntax_error(lexer, None, lexer.token_start_line, lexer.token_start_column, &text, e)
    }
}


impl<'b> Lexer for BaseLexer<'b> {
    fn set_channel(&mut self, v: isize) {
        self.channel = v;
    }

    fn push_mode(&mut self, _m: isize) {
        self.mode_stack.push(self.mode);
        self.mode = _m;
    }

    fn pop_mode(&mut self) -> Option<isize> {
        self.mode_stack.pop().map(|mode| {
            self.mode = mode;
            mode
        })
    }

    fn set_type(&mut self, t: isize) {
        self.token_type = t;
    }

    fn set_mode(&mut self, m: isize) {
        self.mode = m;
    }

    fn more(&mut self) {
        self.set_type(LEXER_MORE)
    }

    fn skip(&mut self) {
        self.set_type(LEXER_SKIP)
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}
