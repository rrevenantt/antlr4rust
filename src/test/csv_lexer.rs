// Generated from /home/rrevenantt/CSV.g4 by ANTLR 4.7.1

use crate::atn::ATN;
use crate::char_stream::CharStream;
use crate::lexer::BaseLexer;
use crate::atn_deserializer::ATNDeserializer;
use crate::dfa::DFA;
use crate::lexer_atn_simulator::LexerATNSimulator;
use crate::prediction_context::PredictionContextCache;
use crate::recognizer::Recognizer;

use crate::error_listener::ErrorListener;
use std::sync::Arc;
use crate::token_source::TokenSource;
use std::cell::RefCell;
use crate::common_token_factory::TokenFactory;
use crate::token::Token;

const T__0: isize = 1;
const T__1: isize = 2;
const T__2: isize = 3;
const TEXT: isize = 4;
const STRING: isize = 5;
const channelNames: [&'static str; 0 + 2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

const modeNames: [&'static str; 1] = [
    "DEFAULT_MODE"
];

const ruleNames: [&'static str; 5] = [
    "T__0", "T__1", "T__2", "TEXT", "STRING"
];

const _LITERAL_NAMES: [Option<&'static str>; 4] = [
    None, Some("','"), Some("'\r'"), Some("'\n'")
];
const _SYMBOLIC_NAMES: [Option<&'static str>; 6] = [
    None, None, None, None, Some("TEXT"), Some("STRING")
];
lazy_static! {
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
	//	static ref VOCABULARY :Vocabulary = VocabularyImpl::new(_LITERAL_NAMES, _SYMBOLIC_NAMES);
	}


pub struct CSVLexer<'a> {
    base: BaseLexer<'a>,
//	static { RuntimeMetaData.checkVersion("4.7.1", RuntimeMetaData.VERSION); }

}

impl<'b> Recognizer for CSVLexer<'b> {
    fn get_atn(&self) -> Arc<ATN> {
        _ATN.clone()
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "CSV.g4"
    }

    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }
    fn get_state(&self) -> isize {
        unimplemented!()
    }

    fn set_state(&mut self, _v: isize) {
        unimplemented!()
    }

    fn add_error_listener(&mut self, _listener: Box<ErrorListener>) {
        self.base.add_error_listener(_listener);
    }

    fn remove_error_listeners(&self) {
        unimplemented!()
    }

    fn get_error_listener_dispatch(&self) -> Box<ErrorListener> {
        unimplemented!()
    }
}

//impl<'a> ATNHolder for CSVLexer<'a>{
//	fn get_atn(&self) -> &ATN { &_ATN }
//
//}
impl TokenSource for CSVLexer<'_> {
    fn next_token(&mut self) -> Box<dyn Token> {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&self) -> &RefCell<Box<dyn CharStream>> {
        self.base.get_input_stream()
    }

    fn get_token_factory(&self) -> &TokenFactory {
        self.base.get_token_factory()
    }
}


impl<'a> CSVLexer<'a> {
    pub fn new(input: Box<dyn CharStream>) -> CSVLexer<'a> {
        CSVLexer {
            base: BaseLexer::new_base_lexer(
                input,
                Box::new(LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                )),
            ),
        }
    }


//	pub fn get_grammar_file_name() -> &'static str{ "CSV.g4"}
//
//	pub fn get_rule_names() -> &'static [&'static str] {&ruleNames}
//
//	pub fn get_serialized_atn() -> &'static str { _serializedATN }
//
//	//pub fn get_vocabulary() -> &Vocabulary {&VOCABULARY }
//
//	pub fn get_channel_names() -> &'static [&'static str] {&channelNames}
//
//	pub fn get_mode_names() -> &'static [&'static str] {&modeNames}
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
		\x07\x23\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\
		\x03\x05\x06\x05\x15\x0a\x05\x0d\x05\x0e\x05\x16\x03\x06\x03\x06\x03\x06\
		\x03\x06\x07\x06\x1d\x0a\x06\x0c\x06\x0e\x06\x20\x0b\x06\x03\x06\x03\x06\
		\x02\x02\x07\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x03\x02\x04\x06\x02\
		\x0c\x0c\x0f\x0f\x24\x24\x2e\x2e\x03\x02\x24\x24\x02\x25\x02\x03\x03\x02\
		\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\
		\x02\x02\x02\x0b\x03\x02\x02\x02\x03\x0d\x03\x02\x02\x02\x05\x0f\x03\x02\
		\x02\x02\x07\x11\x03\x02\x02\x02\x09\x14\x03\x02\x02\x02\x0b\x18\x03\x02\
		\x02\x02\x0d\x0e\x07\x2e\x02\x02\x0e\x04\x03\x02\x02\x02\x0f\x10\x07\x0f\
		\x02\x02\x10\x06\x03\x02\x02\x02\x11\x12\x07\x0c\x02\x02\x12\x08\x03\x02\
		\x02\x02\x13\x15\x0a\x02\x02\x02\x14\x13\x03\x02\x02\x02\x15\x16\x03\x02\
		\x02\x02\x16\x14\x03\x02\x02\x02\x16\x17\x03\x02\x02\x02\x17\x0a\x03\x02\
		\x02\x02\x18\x1e\x07\x24\x02\x02\x19\x1a\x07\x24\x02\x02\x1a\x1d\x07\x24\
		\x02\x02\x1b\x1d\x0a\x03\x02\x02\x1c\x19\x03\x02\x02\x02\x1c\x1b\x03\x02\
		\x02\x02\x1d\x20\x03\x02\x02\x02\x1e\x1c\x03\x02\x02\x02\x1e\x1f\x03\x02\
		\x02\x02\x1f\x21\x03\x02\x02\x02\x20\x1e\x03\x02\x02\x02\x21\x22\x07\x24\
		\x02\x02\x22\x0c\x03\x02\x02\x02\x06\x02\x16\x1c\x1e\x02";

