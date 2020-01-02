// Generated from SimpleLR.g4 by ANTLR 4.7.1
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(try_blocks)]

use std::cell::RefCell;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, ParserRuleContextType};
use antlr_rust::prediction_context::PredictionContextCache;
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::TOKEN_EOF;
use antlr_rust::token_source::TokenSource;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::TerminalNode;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};

use super::simplelrlistener::*;

pub const ID: isize = 1;
pub const WS: isize = 2;
pub const RULE_s: usize = 0;
pub const RULE_a: usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "s", "a"
];

pub const _LITERAL_NAMES: [Option<&'static str>; 0] = [];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 3] = [
    None, Some("ID"), Some("WS")
];
lazy_static! {
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}

pub struct SimpleLRParser {
    base: BaseParser<SimpleLRParserExt, dyn SimpleLRListener, SimpleLRListenerCaller>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    err_handler: Box<dyn ErrorStrategy>,

}

impl SimpleLRParser {
    pub fn get_serialized_atn() -> &'static str { unimplemented!() }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy>) {
        self.err_handler = strategy
    }


    pub fn new(input: Box<dyn TokenStream>) -> Self {
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                SimpleLRParserExt,
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: Box::new(DefaultErrorStrategy::new()),
        }
    }
}

impl Deref for SimpleLRParser {
    type Target = BaseParser<SimpleLRParserExt, dyn SimpleLRListener, SimpleLRListenerCaller>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for SimpleLRParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SimpleLRParserExt;

impl ParserRecog for SimpleLRParserExt {}

impl Recognizer for SimpleLRParserExt {
    fn get_grammar_file_name(&self) -> &str { "SimpleLR.g4" }

    fn get_rule_names(&self) -> &[&str] { &ruleNames }

    fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}

impl Actions for SimpleLRParserExt {
    type Recog = dyn Parser;
    fn sempred(&self, _localctx: Option<&dyn ParserRuleContext>, rule_index: isize, pred_index: isize,
               recog: &<Self as Actions>::Recog,
    ) -> bool {
        match rule_index {
            1 => self.a_sempred(_localctx, pred_index, recog),
            _ => true
        }
    }
}

impl SimpleLRParserExt {
    fn a_sempred(&self, _localctx: Option<&dyn ParserRuleContext>, pred_index: isize,
                 recog: &<Self as Actions>::Recog,
    ) -> bool {
        match pred_index {
            0 => {
                recog.precpred(None, 2)
            }
            _ => true
        }
    }
}

pub struct SContext {
    base: BaseParserRuleContext<SContextExt>,

}

impl Deref for SContext {
    type Target = BaseParserRuleContext<SContextExt>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for SContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SContextExt {}

impl CustomRuleContext for SContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_s
    }
}


impl SContext {
    pub fn a(&self) -> &AContext {
        unimplemented!()
        //getRuleContext(AContext.class,0)
    }
    pub fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<dyn ParserRuleContext> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, SContextExt {}),
        )
    }
}


impl SimpleLRParser {
    pub fn s(&mut self) -> Result<Rc<dyn ParserRuleContext>/*SContext*/, ANTLRError> {
        let _parentctx = self.ctx.take();
        let mut _localctx = SContext::new(_parentctx.clone(), self.base.get_state());
        //let mut _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        //self.ctx = Some(_localctx);
        //let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx.clone(), 0, RULE_s);
        let result: Result<(), ANTLRError> = try {
            self.base.enter_outer_alt(None, 1);
            {
                self.base.set_state(4);
                self.a_rec(0)?;
            }
            /*_ctx.stop = input.lt(-1);*/
            println!("test");
        };
        match result {
            Ok(_) => {},
            Err(ref re) => {
                //_localctx.exception = re;
                self.err_handler.report_error(&mut self.base, re);
                self.err_handler.recover(&mut self.base, re);
            }
        }
        self.base.exit_rule();

        Ok(_localctx)
    }
}

pub struct AContext {
    base: BaseParserRuleContext<AContextExt>,

}

impl Deref for AContext {
    type Target = BaseParserRuleContext<AContextExt>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for AContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct AContextExt {}

impl CustomRuleContext for AContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_a
    }
}


impl AContext {
    pub fn ID(&self) -> &TerminalNode {
        unimplemented!()
        //getToken(SimpleLRParser.ID, 0)
    }
    pub fn a(&self) -> &AContext {
        unimplemented!()
        //getRuleContext(AContext.class,0)
    }
    pub fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<dyn ParserRuleContext> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, AContextExt {}),
        )
    }
}

impl SimpleLRParser {
    pub fn a(&mut self) -> Result<Rc<dyn ParserRuleContext>/*AContext*/, ANTLRError> {
        self.a_rec(0)
    }

    fn a_rec(&mut self, _p: isize) -> Result<Rc<dyn ParserRuleContext>/*AContext*/, ANTLRError> {
        let _parentctx = self.ctx.take();
        let _parentState = self.base.get_state();
        let mut _localctx = AContext::new(_parentctx.clone(), self.base.get_state());
        //self.ctx = Some(_localctx);
        //let mut _localctx = self.ctx.as_deref_mut().unwrap();
        let mut _prevctx = _localctx.clone();
        let _startState = 2;
        self.base.enter_recursion_rule(_localctx.clone(), 2, RULE_a, _p);
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            self.base.enter_outer_alt(None, 1);
            {
                {
                    self.base.set_state(7);
                    self.base.match_token(ID, self.err_handler.as_mut())?;
                }
                /*_ctx.stop = input.lt(-1);*/
                self.base.set_state(13);
                self.err_handler.sync(&mut self.base)?;
                _alt = self.interpreter.adaptive_predict(0, &mut self.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        self.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                _localctx = AContext::new(_parentctx.clone(), _parentState);
                                self.push_new_recursion_context(_localctx.clone(), _startState, RULE_a);
                                self.base.set_state(9);
                                if !((|recog: &mut dyn Parser| -> bool{ recog.precpred(None, 2) })(&mut self.base)) {
                                    Err(FailedPredicateError::new(&mut self.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
                                }
                                self.base.set_state(10);
                                self.base.match_token(ID, self.err_handler.as_mut())?;
                            }
                        }
                    }
                    self.base.set_state(15);
                    self.err_handler.sync(&mut self.base)?;
                    _alt = self.interpreter.adaptive_predict(0, &mut self.base)?;
                }
            }
        };
        match result {
            Ok(_) => {},
            Err(ref re) => {
                //_localctx.exception = re;
                self.err_handler.report_error(&mut self.base, re);
                self.err_handler.recover(&mut self.base, re);
            }
        }
        self.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
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
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x04\x13\x04\x02\x09\x02\x04\x03\x09\x03\x03\x02\x03\x02\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x07\x03\x0e\x0a\x03\x0c\x03\x0e\x03\x11\x0b\x03\
	\x03\x03\x02\x03\x04\x04\x02\x04\x02\x02\x02\x11\x02\x06\x03\x02\x02\x02\
	\x04\x08\x03\x02\x02\x02\x06\x07\x05\x04\x03\x02\x07\x03\x03\x02\x02\x02\
	\x08\x09\x08\x03\x01\x02\x09\x0a\x07\x03\x02\x02\x0a\x0f\x03\x02\x02\x02\
	\x0b\x0c\x0c\x04\x02\x02\x0c\x0e\x07\x03\x02\x02\x0d\x0b\x03\x02\x02\x02\
	\x0e\x11\x03\x02\x02\x02\x0f\x0d\x03\x02\x02\x02\x0f\x10\x03\x02\x02\x02\
	\x10\x05\x03\x02\x02\x02\x11\x0f\x03\x02\x02\x02\x03\x0f";


