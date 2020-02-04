// Generated from Labels.g4 by ANTLR 4.7.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use antlr_rust::atn::ATN;
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::char_stream::CharStream;
use antlr_rust::common_token_factory::TokenFactory;
use antlr_rust::dfa::DFA;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use antlr_rust::parser_rule_context::{cast, LexerContext, ParserRuleContext};
use antlr_rust::prediction_context::PredictionContextCache;
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::BaseRuleContext;
use antlr_rust::token::*;
use antlr_rust::token_source::TokenSource;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const ID: isize = 7;
pub const INT: isize = 8;
pub const WS: isize = 9;
pub const channelNames: [&'static str; 0 + 2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str; 1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str; 9] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "ID", "INT", "WS"
];


pub const _LITERAL_NAMES: [Option<&'static str>; 7] = [
    None, Some("'*'"), Some("'+'"), Some("'('"), Some("')'"), Some("'++'"),
    Some("'--'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 10] = [
    None, None, None, None, None, None, None, Some("ID"), Some("INT"), Some("WS")
];
lazy_static! {
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub struct LabelsLexer {
    base: BaseLexer<LabelsLexerActions>,
//	static { RuntimeMetaData.checkVersion("4.7.2", RuntimeMetaData.VERSION); }
}

impl Deref for LabelsLexer {
    type Target = BaseLexer<LabelsLexerActions>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for LabelsLexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}


impl LabelsLexer {
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn add_error_listener(&mut self, _listener: Box<ErrorListener>) {
        self.base.add_error_listener(_listener);
    }

    fn remove_error_listeners(&mut self) {
        self.base.remove_error_listeners()
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "LabelsLexer.g4"
    }

    pub fn new(input: Box<dyn CharStream>) -> Self {
        Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
                Box::new(LabelsLexerActions {}),
            )
        }
    }
}

pub struct LabelsLexerActions {}

impl LabelsLexerActions {}

impl LexerRecog for LabelsLexerActions {}

impl Recognizer for LabelsLexerActions {}

impl Actions for LabelsLexerActions {
    type Recog = BaseLexer<LabelsLexerActions>;
}

impl LabelsLexerActions {}

impl TokenSource for LabelsLexer {
    fn next_token(&mut self) -> Box<dyn Token> {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> &mut dyn CharStream {
        self.base.get_input_stream()
    }

    fn get_token_factory(&self) -> &dyn TokenFactory {
        self.base.get_token_factory()
    }
}



lazy_static! {
	    static ref _ATN: Arc<ATN> =
	        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
	    static ref _decision_to_DFA: Arc<Vec<DFA>> = {
	        let mut dfa = Vec::new();
	        let size = _ATN.decision_to_state.len();
	        for i in 0..size {
	            dfa.push(DFA::new(
	                _ATN.clone(),
	                _ATN.get_decision_state(i),
	                i as isize,
	            ))
	        }
	        Arc::new(dfa)
	    };
	}



const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x0b\x31\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\x03\x05\
		\x03\x05\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x08\x06\x08\
		\x25\x0a\x08\x0d\x08\x0e\x08\x26\x03\x09\x06\x09\x2a\x0a\x09\x0d\x09\x0e\
		\x09\x2b\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x02\x02\x0b\x03\x03\x05\x04\x07\
		\x05\x09\x06\x0b\x07\x0d\x08\x0f\x09\x11\x0a\x13\x0b\x03\x02\x03\x04\x02\
		\x0c\x0c\x22\x22\x02\x32\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\
		\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\
		\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\
		\x02\x13\x03\x02\x02\x02\x03\x15\x03\x02\x02\x02\x05\x17\x03\x02\x02\x02\
		\x07\x19\x03\x02\x02\x02\x09\x1b\x03\x02\x02\x02\x0b\x1d\x03\x02\x02\x02\
		\x0d\x20\x03\x02\x02\x02\x0f\x24\x03\x02\x02\x02\x11\x29\x03\x02\x02\x02\
		\x13\x2d\x03\x02\x02\x02\x15\x16\x07\x2c\x02\x02\x16\x04\x03\x02\x02\x02\
		\x17\x18\x07\x2d\x02\x02\x18\x06\x03\x02\x02\x02\x19\x1a\x07\x2a\x02\x02\
		\x1a\x08\x03\x02\x02\x02\x1b\x1c\x07\x2b\x02\x02\x1c\x0a\x03\x02\x02\x02\
		\x1d\x1e\x07\x2d\x02\x02\x1e\x1f\x07\x2d\x02\x02\x1f\x0c\x03\x02\x02\x02\
		\x20\x21\x07\x2f\x02\x02\x21\x22\x07\x2f\x02\x02\x22\x0e\x03\x02\x02\x02\
		\x23\x25\x04\x63\x7c\x02\x24\x23\x03\x02\x02\x02\x25\x26\x03\x02\x02\x02\
		\x26\x24\x03\x02\x02\x02\x26\x27\x03\x02\x02\x02\x27\x10\x03\x02\x02\x02\
		\x28\x2a\x04\x32\x3b\x02\x29\x28\x03\x02\x02\x02\x2a\x2b\x03\x02\x02\x02\
		\x2b\x29\x03\x02\x02\x02\x2b\x2c\x03\x02\x02\x02\x2c\x12\x03\x02\x02\x02\
		\x2d\x2e\x09\x02\x02\x02\x2e\x2f\x03\x02\x02\x02\x2f\x30\x08\x0a\x02\x02\
		\x30\x14\x03\x02\x02\x02\x05\x02\x26\x2b\x03\x08\x02\x02";

