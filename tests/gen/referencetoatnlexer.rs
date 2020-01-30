// Generated from ReferenceToATN.g4 by ANTLR 4.7.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

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

pub const ID: isize = 1;
pub const ATN: isize = 2;
pub const WS: isize = 3;
pub const channelNames: [&'static str; 0 + 2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str; 1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str; 3] = [
    "ID", "ATN", "WS"
];


pub const _LITERAL_NAMES: [Option<&'static str>; 0] = [];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 4] = [
    None, Some("ID"), Some("ATN"), Some("WS")
];
lazy_static! {
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub struct ReferenceToATNLexer {
    base: BaseLexer<ReferenceToATNLexerActions>,
//	static { RuntimeMetaData.checkVersion("4.7.2", RuntimeMetaData.VERSION); }
}

impl Deref for ReferenceToATNLexer {
    type Target = BaseLexer<ReferenceToATNLexerActions>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for ReferenceToATNLexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}


impl ReferenceToATNLexer {
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
        "ReferenceToATNLexer.g4"
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
                Box::new(ReferenceToATNLexerActions {}),
            )
        }
    }
}

pub struct ReferenceToATNLexerActions {}

impl ReferenceToATNLexerActions {}

impl LexerRecog for ReferenceToATNLexerActions {}

impl Recognizer for ReferenceToATNLexerActions {}

impl Actions for ReferenceToATNLexerActions {
    type Recog = BaseLexer<ReferenceToATNLexerActions>;
}

impl ReferenceToATNLexerActions {}

impl TokenSource for ReferenceToATNLexer {
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
		\x05\x17\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x03\x02\
		\x06\x02\x0b\x0a\x02\x0d\x02\x0e\x02\x0c\x03\x03\x06\x03\x10\x0a\x03\x0d\
		\x03\x0e\x03\x11\x03\x04\x03\x04\x03\x04\x03\x04\x02\x02\x05\x03\x03\x05\
		\x04\x07\x05\x03\x02\x03\x04\x02\x0c\x0c\x22\x22\x02\x18\x02\x03\x03\x02\
		\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x03\x0a\x03\x02\
		\x02\x02\x05\x0f\x03\x02\x02\x02\x07\x13\x03\x02\x02\x02\x09\x0b\x04\x63\
		\x7c\x02\x0a\x09\x03\x02\x02\x02\x0b\x0c\x03\x02\x02\x02\x0c\x0a\x03\x02\
		\x02\x02\x0c\x0d\x03\x02\x02\x02\x0d\x04\x03\x02\x02\x02\x0e\x10\x04\x32\
		\x3b\x02\x0f\x0e\x03\x02\x02\x02\x10\x11\x03\x02\x02\x02\x11\x0f\x03\x02\
		\x02\x02\x11\x12\x03\x02\x02\x02\x12\x06\x03\x02\x02\x02\x13\x14\x09\x02\
		\x02\x02\x14\x15\x03\x02\x02\x02\x15\x16\x08\x04\x02\x02\x16\x08\x03\x02\
		\x02\x02\x05\x02\x0c\x11\x03\x08\x02\x02";

