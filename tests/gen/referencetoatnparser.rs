// Generated from ReferenceToATN.g4 by ANTLR 4.7.1
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

use super::referencetoatnlistener::*;

pub const ID: isize = 1;
pub const ATN: isize = 2;
pub const WS: isize = 3;
pub const RULE_a: usize = 0;
pub const ruleNames: [&'static str; 1] = [
    "a"
];

pub const _LITERAL_NAMES: [Option<&'static str>; 0] = [];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 4] = [
    None, Some("ID"), Some("ATN"), Some("WS")
];
lazy_static! {
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}

pub struct ReferenceToATNParser {
    base: BaseParser<ReferenceToATNParserExt, dyn ReferenceToATNListener, ReferenceToATNListenerCaller>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    err_handler: Box<dyn ErrorStrategy>,

}

impl ReferenceToATNParser {
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
                ReferenceToATNParserExt,
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: Box::new(DefaultErrorStrategy::new()),
        }
    }
}

impl Deref for ReferenceToATNParser {
    type Target = BaseParser<ReferenceToATNParserExt, dyn ReferenceToATNListener, ReferenceToATNListenerCaller>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for ReferenceToATNParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ReferenceToATNParserExt;

impl ParserRecog for ReferenceToATNParserExt {}

impl Recognizer for ReferenceToATNParserExt {
    fn get_grammar_file_name(&self) -> &str { "ReferenceToATN.g4" }

    fn get_rule_names(&self) -> &[&str] { &ruleNames }

    fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}

impl Actions for ReferenceToATNParserExt {
    type Recog = dyn Parser;
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
    pub fn ATNs(&self) -> Vec<&TerminalNode> {
        unimplemented!()
        //getTokens(ReferenceToATNParser.ATN)
    }
    pub fn ATN(&self, i: usize) -> &TerminalNode {
        unimplemented!()
        //getToken(ReferenceToATNParser.ATN, i)
    }
    pub fn IDs(&self) -> Vec<&TerminalNode> {
        unimplemented!()
        //getTokens(ReferenceToATNParser.ID)
    }
    pub fn ID(&self, i: usize) -> &TerminalNode {
        unimplemented!()
        //getToken(ReferenceToATNParser.ID, i)
    }
    pub fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<dyn ParserRuleContext> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, AContextExt {}),
        )
    }
}


impl ReferenceToATNParser {
    pub fn a(&mut self) -> Result<Rc<dyn ParserRuleContext>/*AContext*/, ANTLRError> {
        let _parentctx = self.ctx.take();
        let mut _localctx = AContext::new(_parentctx.clone(), self.base.get_state());
        //let mut _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        //self.ctx = Some(_localctx);
        //let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx.clone(), 0, RULE_a);
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            self.base.enter_outer_alt(None, 1);
            {
                self.base.set_state(5);
                self.err_handler.sync(&mut self.base)?;
                _alt = self.interpreter.adaptive_predict(0, &mut self.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                self.base.set_state(2);
                                _la = self.base.input.la(1);
                                if { !(_la == ID || _la == ATN) } {
                                    self.err_handler.recover_inline(&mut self.base);
                                } else {
                                    if self.base.input.la(1) == TOKEN_EOF { self.base.matched_eof = true };
                                    self.err_handler.report_match(&mut self.base);
                                    self.base.consume(self.err_handler.as_mut());
                                }
                            }
                        }
                    }
                    self.base.set_state(7);
                    self.err_handler.sync(&mut self.base)?;
                    _alt = self.interpreter.adaptive_predict(0, &mut self.base)?;
                }
                self.base.set_state(9);
                self.err_handler.sync(&mut self.base)?;
                _la = self.base.input.la(1);
                if _la == ATN {
                    {
                        self.base.set_state(8);
                        self.base.match_token(ATN, self.err_handler.as_mut())?;
                    }
                }

                println!("{}", {
                    let temp = self.base.input.index() - 1;
                    self.base.input.get_text_from_interval(self.get_parser_rule_context().get_start(), temp)
                });
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
        self.base.exit_rule();

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
	\x05\x10\x04\x02\x09\x02\x03\x02\x07\x02\x06\x0a\x02\x0c\x02\x0e\x02\x09\
	\x0b\x02\x03\x02\x05\x02\x0c\x0a\x02\x03\x02\x03\x02\x03\x02\x02\x02\x03\
	\x02\x02\x03\x03\x02\x03\x04\x02\x10\x02\x07\x03\x02\x02\x02\x04\x06\x09\
	\x02\x02\x02\x05\x04\x03\x02\x02\x02\x06\x09\x03\x02\x02\x02\x07\x05\x03\
	\x02\x02\x02\x07\x08\x03\x02\x02\x02\x08\x0b\x03\x02\x02\x02\x09\x07\x03\
	\x02\x02\x02\x0a\x0c\x07\x04\x02\x02\x0b\x0a\x03\x02\x02\x02\x0b\x0c\x03\
	\x02\x02\x02\x0c\x0d\x03\x02\x02\x02\x0d\x0e\x08\x02\x01\x02\x0e\x03\x03\
	\x02\x02\x02\x04\x07\x0b";


