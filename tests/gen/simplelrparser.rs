// Generated from SimpleLR.g4 by ANTLR 4.7.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
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
use antlr_rust::parser_rule_context::{BaseParserRuleContext, cast, cast_mut, ParserRuleContext, ParserRuleContextType};
use antlr_rust::prediction_context::PredictionContextCache;
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_source::TokenSource;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::{ParseTree, TerminalNode};
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


type BaseParserType = BaseParser<SimpleLRParserExt, dyn SimpleLRListener>;

pub struct SimpleLRParser {
    base: BaseParserType,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy>,
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
                SimpleLRParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: Box::new(DefaultErrorStrategy::new()),
        }
    }
}

impl Deref for SimpleLRParser {
    type Target = BaseParserType;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for SimpleLRParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SimpleLRParserExt {}

impl SimpleLRParserExt {}

impl ParserRecog for SimpleLRParserExt {}

impl Recognizer for SimpleLRParserExt {
    fn get_grammar_file_name(&self) -> &str { "SimpleLR.g4" }

    fn get_rule_names(&self) -> &[&str] { &ruleNames }

    fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}

impl Actions for SimpleLRParserExt {
    type Recog = BaseParserType;
    fn sempred(_localctx: &dyn ParserRuleContext, rule_index: isize, pred_index: isize,
               recog: &mut <Self as Actions>::Recog,
    ) -> bool {
        match rule_index {
            1 => Self::a_sempred(cast::<_, AContext>(_localctx), pred_index, recog),
            _ => true
        }
    }
}

impl SimpleLRParserExt {
    fn a_sempred(_localctx: &AContext, pred_index: isize,
                 recog: &mut <Self as Actions>::Recog,
    ) -> bool {
        match pred_index {
            0 => {
                recog.precpred(None, 2)
            }
            _ => true
        }
    }
}

//------------------- s ----------------
pub type SContextAll = SContext;


pub type SContext = BaseParserRuleContext<SContextExt>;

#[derive(Clone)]
pub struct SContextExt {}

impl CustomRuleContext for SContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_s
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn SimpleLRListener>>()
            .map(|it| it.enter_s(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn SimpleLRListener>>()
            .map(|it| it.exit_s(ctx));
    }
}

impl SContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<SContextAll> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, SContextExt {}),
        )
    }
}

pub trait SContextAttrs: ParserRuleContext + BorrowMut<SContextExt> {
    fn a(&self) -> Rc<AContextAll> where Self: Sized {
        self.child_of_type(0)
    }
}

impl SContextAttrs for SContext {}

//impl SContext{

//}

impl SimpleLRParser {
    pub fn s(&mut self)
             -> Result<Rc<SContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_s);
        let mut _localctx: Rc<SContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {

            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule a*/
                recog.base.set_state(4);
                recog.a_rec(0)?;
            }
            let tmp = recog.input.lt(-1).map(Token::to_owned);
            recog.ctx.as_ref().unwrap().set_stop(tmp);
            println!("test");
        };
        match result {
            Ok(_) => {},
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

//------------------- a ----------------
pub type AContextAll = AContext;


pub type AContext = BaseParserRuleContext<AContextExt>;

#[derive(Clone)]
pub struct AContextExt {}

impl CustomRuleContext for AContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_a
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn SimpleLRListener>>()
            .map(|it| it.enter_a(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn SimpleLRListener>>()
            .map(|it| it.exit_a(ctx));
    }
}

impl AContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<AContextAll> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, AContextExt {}),
        )
    }
}

pub trait AContextAttrs: ParserRuleContext + BorrowMut<AContextExt> {
    fn ID(&self) -> Option<Rc<TerminalNode>> where Self: Sized {
        self.get_token(ID, 0)
    }
    fn a(&self) -> Rc<AContextAll> where Self: Sized {
        self.child_of_type(0)
    }
}

impl AContextAttrs for AContext {}

//impl AContext{

//}

impl SimpleLRParser {
    pub fn a(&mut self)
             -> Result<Rc<AContextAll>, ANTLRError> {
        self.a_rec(0)
    }

    fn a_rec(&mut self, _p: isize)
             -> Result<Rc<AContextAll>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = AContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_recursion_rule(_localctx.clone(), 2, RULE_a, _p);
        let mut _localctx: Rc<AContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 2;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    recog.base.set_state(7);
                    recog.base.match_token(ID, recog.err_handler.as_mut())?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(13);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                /*recRuleAltStartAction*/
                                let mut tmp = AContextExt::new(_parentctx.clone(), _parentState);
                                recog.push_new_recursion_context(tmp.clone(), _startState, RULE_a);
                                _localctx = tmp;
                                recog.base.set_state(9);
                                if !({ recog.precpred(None, 2) }) {
                                    Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
                                }
                                recog.base.set_state(10);
                                recog.base.match_token(ID, recog.err_handler.as_mut())?;
                            }
                        }
                    }
                    recog.base.set_state(15);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {},
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

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


