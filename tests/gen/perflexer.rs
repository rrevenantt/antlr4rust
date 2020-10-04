// Generated from Perf.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::token_source::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::lazy_static;

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const T__2:isize=3; 
	pub const T__3:isize=4; 
	pub const T__4:isize=5; 
	pub const T__5:isize=6; 
	pub const T__6:isize=7; 
	pub const T__7:isize=8; 
	pub const T__8:isize=9; 
	pub const T__9:isize=10; 
	pub const ID:isize=11; 
	pub const WS:isize=12;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;12] = [
		"T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
		"T__9", "ID", "WS"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
		None, Some("';'"), Some("'.'"), Some("'not'"), Some("'and'"), Some("'or'"), 
		Some("'('"), Some("')'"), Some("'?'"), Some("':'"), Some("'between'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;13]  = [
		None, None, None, None, None, None, None, None, None, None, None, Some("ID"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseParserRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct PerfLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,PerfLexerActions,Input,LocalTokenFactory<'input>>,
//	static { RuntimeMetaData.checkVersion("4.8", RuntimeMetaData.VERSION); }
}

impl<'input, Input:CharStream<From<'input> >> Deref for PerfLexer<'input,Input>{
	type Target = BaseLexer<'input,PerfLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for PerfLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> PerfLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "PerfLexer.g4"
    }

	pub fn new_with_token_factory(input: Box<Input>,tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","2");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				PerfLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> PerfLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Box<Input>) -> Self{
		PerfLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct PerfLexerActions {
}

impl PerfLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,PerfLexerActions,Input,LocalTokenFactory<'input>>> for PerfLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> PerfLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,PerfLexerActions,Input,LocalTokenFactory<'input>>> for PerfLexerActions{
}
impl<'input> TokenAware<'input> for PerfLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenAware<'input> for PerfLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for PerfLexer<'input,Input>{

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
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



	const _serializedATN:&'static str =
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x0e\x48\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x03\x02\
		\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\
		\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x08\x03\x08\
		\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
		\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x07\x0c\x3d\x0a\x0c\x0c\x0c\x0e\
		\x0c\x40\x0b\x0c\x03\x0d\x06\x0d\x43\x0a\x0d\x0d\x0d\x0e\x0d\x44\x03\x0d\
		\x03\x0d\x02\x02\x0e\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\x0f\
		\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x03\x02\x05\x05\x02\x43\x5c\
		\x61\x61\x63\x7c\x06\x02\x32\x3b\x43\x5c\x61\x61\x63\x7c\x05\x02\x0b\x0c\
		\x0e\x0f\x22\x22\x02\x49\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\
		\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\
		\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\
		\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\x03\x02\x02\x02\
		\x02\x19\x03\x02\x02\x02\x03\x1b\x03\x02\x02\x02\x05\x1d\x03\x02\x02\x02\
		\x07\x1f\x03\x02\x02\x02\x09\x23\x03\x02\x02\x02\x0b\x27\x03\x02\x02\x02\
		\x0d\x2a\x03\x02\x02\x02\x0f\x2c\x03\x02\x02\x02\x11\x2e\x03\x02\x02\x02\
		\x13\x30\x03\x02\x02\x02\x15\x32\x03\x02\x02\x02\x17\x3a\x03\x02\x02\x02\
		\x19\x42\x03\x02\x02\x02\x1b\x1c\x07\x3d\x02\x02\x1c\x04\x03\x02\x02\x02\
		\x1d\x1e\x07\x30\x02\x02\x1e\x06\x03\x02\x02\x02\x1f\x20\x07\x70\x02\x02\
		\x20\x21\x07\x71\x02\x02\x21\x22\x07\x76\x02\x02\x22\x08\x03\x02\x02\x02\
		\x23\x24\x07\x63\x02\x02\x24\x25\x07\x70\x02\x02\x25\x26\x07\x66\x02\x02\
		\x26\x0a\x03\x02\x02\x02\x27\x28\x07\x71\x02\x02\x28\x29\x07\x74\x02\x02\
		\x29\x0c\x03\x02\x02\x02\x2a\x2b\x07\x2a\x02\x02\x2b\x0e\x03\x02\x02\x02\
		\x2c\x2d\x07\x2b\x02\x02\x2d\x10\x03\x02\x02\x02\x2e\x2f\x07\x41\x02\x02\
		\x2f\x12\x03\x02\x02\x02\x30\x31\x07\x3c\x02\x02\x31\x14\x03\x02\x02\x02\
		\x32\x33\x07\x64\x02\x02\x33\x34\x07\x67\x02\x02\x34\x35\x07\x76\x02\x02\
		\x35\x36\x07\x79\x02\x02\x36\x37\x07\x67\x02\x02\x37\x38\x07\x67\x02\x02\
		\x38\x39\x07\x70\x02\x02\x39\x16\x03\x02\x02\x02\x3a\x3e\x09\x02\x02\x02\
		\x3b\x3d\x09\x03\x02\x02\x3c\x3b\x03\x02\x02\x02\x3d\x40\x03\x02\x02\x02\
		\x3e\x3c\x03\x02\x02\x02\x3e\x3f\x03\x02\x02\x02\x3f\x18\x03\x02\x02\x02\
		\x40\x3e\x03\x02\x02\x02\x41\x43\x09\x04\x02\x02\x42\x41\x03\x02\x02\x02\
		\x43\x44\x03\x02\x02\x02\x44\x42\x03\x02\x02\x02\x44\x45\x03\x02\x02\x02\
		\x45\x46\x03\x02\x02\x02\x46\x47\x08\x0d\x02\x02\x47\x1a\x03\x02\x02\x02\
		\x05\x02\x3e\x44\x03\x08\x02\x02";
