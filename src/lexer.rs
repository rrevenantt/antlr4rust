use crate::char_stream::CharStream;
//use lexer_atn_simulator::ILexerATNSimulator;
use crate::common_token_factory::TokenFactory;
use crate::error_listener::ErrorListener;
use crate::errors::ANTLRError;
use crate::lexer_atn_simulator::ILexerATNSimulator;
use crate::recognizer::BaseRecognizer;
use crate::recognizer::Recognizer;

use std::borrow::{Borrow, BorrowMut};

use crate::token::Token;
use crate::token_source::TokenSource;
use std::cell::RefCell;

use std::rc::Rc;
use crate::int_stream::EOF;
use std::time::Duration;

pub trait Lexer: Recognizer + TokenSource {
    fn set_channel(&mut self, v: isize);

    fn push_mode(&mut self, m: isize);

    fn pop_mode(&mut self) -> isize;

    fn set_type(&mut self, t: isize);

    fn set_mode(&mut self, m: isize);

    fn reset(&mut self);
}

pub struct BaseLexer<'b> {
    pub base: BaseRecognizer,

    pub interpreter: Box<ILexerATNSimulator>,
    token_start_char_index: isize,
    token_start_line: isize,
    token_start_column: isize,
    token_type: isize,
    //    virt: Lexer,
    input: Option<RefCell<Box<CharStream>>>,
    factory: &'b TokenFactory,
    //    token_factory_source_pair: Option<(Box<TokenSource>,Box<CharStream>)>,
    token: Option<Box<Token>>,
    hit_eof: bool,
    channel: isize,
    //    thetype: isize,
    //    mode_stack: IntStack,
    mode: isize,
    text: String,
}

pub const LEXER_DEFAULT_MODE: isize = 0;
pub const LEXER_MORE: isize = -2;
pub const LEXER_SKIP: isize = -3;

pub const lexer_default_token_channel: isize = super::token::token_default_channel;
pub const lexer_hidden: isize = super::token::token_hidden_channel;
pub const LEXER_MIN_CHAR_VALUE: isize = 0x0000;
pub const LEXER_MAX_CHAR_VALUE: isize = 0x10FFFF;

impl<'b> BaseLexer<'b> {
    //    fn get_interpreter(&self) -> &ILexerATNSimulator { unimplemented!() }

    fn safe_match(&self) {
        unimplemented!()
    }

    fn set_input_stream(&mut self, input: Box<CharStream>) {
        self.input = Some(RefCell::new(input));
    }

    fn emit_token(&mut self, token: Box<Token>) {
        self.token = Some(token);
    }

    //    fn get_token_source_char_stream_pair(&self) -> * TokenSourceCharStreamPair { unimplemented!() }

    fn emit(&mut self) {
        let token = self.factory.create(
            None,
            self.token_type,
            self.text.clone(),
            self.channel,
            self.token_start_char_index,
            self.get_char_index() - 1,
            self.token_start_line,
            self.token_start_column,
        );
        self.emit_token(token);
    }

    fn emit_eof(&mut self) {
        let token = self.factory.create(
            None,
            super::int_stream::EOF,
            "<EOF>".into(),
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
        self.get_input_stream().borrow().index()
    }

    fn get_text(&self) -> String {
        unimplemented!()
    }

    fn set_text(&self, _text: String) {
        unimplemented!()
    }

    fn get_all_tokens(&self) -> Vec<Box<Token>> {
        unimplemented!()
    }

    fn notify_listeners(_liseners: &Vec<Box<ErrorListener>>, _e: &ANTLRError) {
        for listener in _liseners {
            //            listener.syntax_error()
        }
    }

    fn get_error_display_for_char(&self, _c: char) -> String {
        unimplemented!()
    }

    fn get_char_error_display(&self, _c: char) -> String {
        unimplemented!()
    }

    pub fn new_base_lexer(
        input: Box<CharStream>,
        interp: Box<ILexerATNSimulator>,
    ) -> BaseLexer<'b> {
        BaseLexer {
            input: Some(RefCell::new(input)),
            interpreter: interp,
            base: BaseRecognizer::new_base_recognizer(),
            token_start_char_index: -1,
            token_start_line: -1,
            token_start_column: -1,
            token_type: super::token::token_invalid_type,
            factory: super::common_token_factory::CommonTokenFactoryDEFAULT.as_ref(),
            text: "".into(),
            token: None,
            hit_eof: false,
            channel: super::token::token_default_channel,
            //            token_factory_source_pair: None,
            mode: self::LEXER_DEFAULT_MODE,
        }
    }
}

impl<'b> TokenSource for BaseLexer<'b> {
    fn next_token(&mut self) -> Box<Token> {
        assert!(self.input.is_some());
        let _marker = self.get_input_stream().borrow_mut().mark();
        let mut hit_eof = false;
        'outer: loop {
            if hit_eof {
                self.emit_eof();
                break;
            }
            self.token = None;
            self.channel = lexer_default_token_channel;
            self.token_start_column = self.interpreter.get_char_position_in_line();
            self.token_start_line = self.interpreter.get_line();
            self.text = String::new();
            let index = self.get_input_stream().borrow().index();
            self.token_start_char_index = index;

            'inner: loop {
                let ttype;
                self.token_type = super::token::token_invalid_type;
                {
                    let mut input_container = self.input.as_ref().unwrap().borrow_mut();
                    let input = input_container.as_mut();

                    ttype = match self.interpreter.match_token(input, self.mode) {
                        Ok(ttype) => ttype,
                        Err(err) => {
                            println!("error, recovering");
                            BaseLexer::notify_listeners(&self.base.listeners, &err);
                            self.interpreter.recover(err, input);
                            LEXER_SKIP
                        }
                    };
                }
                if self.get_input_stream().borrow_mut().la(1) == super::int_stream::EOF {
                    hit_eof = true;
                }

                if ttype == LEXER_SKIP {
                    continue 'outer;
                }
                self.token_type = ttype;

                if ttype != LEXER_MORE {
                    break;
                }
            }

            if self.token.is_none() {
                self.emit();
                break;
            }
        }
        self.get_input_stream().borrow_mut().release(_marker);
        self.token.take().unwrap()
    }

    fn more(&mut self) {
        self.set_mode(LEXER_MORE)
    }

    fn skip(&mut self) {
        self.set_mode(LEXER_SKIP)
    }

    fn get_line(&self) -> isize {
        self.token_start_line
    }

    fn get_char_position_in_line(&self) -> isize {
        self.token_start_column
    }

    fn get_input_stream(&self) -> &RefCell<Box<CharStream>> {
        self.input.as_ref().unwrap()
    }

//    fn set_token_factory<'c: 'b>(&mut self, f: &'c TokenFactory) {
//        self.factory = f;
//    }

    fn get_token_factory(&self) -> &TokenFactory {
        self.factory
    }
}

impl<'b> Recognizer for BaseLexer<'b> {
    fn get_state(&self) -> isize {
        self.base.get_state()
    }

    fn set_state(&mut self, v: isize) {
        self.base.set_state(v)
    }

    fn add_error_listener(&mut self, listener: Box<ErrorListener>) {
        self.base.add_error_listener(listener)
    }

    fn remove_error_listeners(&self) {
        self.base.remove_error_listeners()
    }

    fn get_error_listener_dispatch(&self) -> Box<ErrorListener> {
        self.base.get_error_listener_dispatch()
    }
}

impl<'b> Lexer for BaseLexer<'b> {
    fn set_channel(&mut self, v: isize) {
        self.channel = v;
    }

    fn push_mode(&mut self, _m: isize) {
        unimplemented!()
    }

    fn pop_mode(&mut self) -> isize {
        unimplemented!()
    }

    fn set_type(&mut self, t: isize) {
        self.token_type = t;
    }

    fn set_mode(&mut self, m: isize) {
        self.mode = m;
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}
