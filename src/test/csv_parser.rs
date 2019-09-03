// Generated from /home/rrevenantt/CSV.g4 by ANTLR 4.7.1
//extern crate antlr_runtime;

use crate::prediction_context::PredictionContextCache;
use crate::parser::{Parser, BaseParser};
use crate::token_stream::TokenStream;
use crate::token_source::TokenSource;
use crate::parser_atn_simulator::ParserATNSimulator;
use crate::errors::{ANTLRError, NoViableAltError};
use crate::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use std::convert::TryFrom;
use std::sync::Arc;
use crate::recognizer::Recognizer;
use crate::atn_deserializer::ATNDeserializer;
use std::cell::RefCell;
use crate::dfa::DFA;
use crate::atn::ATN;
use crate::error_strategy::{ErrorStrategy, DefaultErrorStrategy};

const T__0: isize = 1;
const T__1: isize = 2;
const T__2: isize = 3;
const TEXT: isize = 4;
const STRING: isize = 5;
const RULE_csvFile: usize = 0;
const RULE_hdr: usize = 1;
const RULE_row: usize = 2;
const RULE_field: usize = 3;
const ruleNames: [&'static str; 4] = [
    "csvFile", "hdr", "row", "field"
];

//lazy_static!{
const _LITERAL_NAMES: [Option<&'static str>; 4] = [
    None, Some("','"), Some("'\r'"), Some("'\n'")
];
const _SYMBOLIC_NAMES: [Option<&'static str>; 6] = [
    None, None, None, None, Some("TEXT"), Some("STRING")
];
lazy_static! {
//	static ref VOCABULARY :Vocabulary = VocabularyImpl::new(_LITERAL_NAMES, _SYMBOLIC_NAMES);
    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
}
//}

pub struct CSVParser {
    base: BaseParser,
    _sharedContextCache: Box<PredictionContextCache>,
    ctx: Option<Box<dyn RuleContext>>,
    err_handler: Box<dyn ErrorStrategy>,
    //		new PredictionContextCache();
}

impl CSVParser {
    pub fn get_grammar_file_name() -> &'static str { "CSV.g4" }

    pub fn get_rule_names() -> &'static [&'static str] { &ruleNames }

    pub fn get_serialized_atn() -> &'static str { unimplemented!() }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy>) {
        self.err_handler = strategy
    }
//	pub fn get_vocabulary() -> &Vocabulary {&VOCABULARY }


    pub fn new(input: Box<dyn TokenStream>) -> Self {
        Self {
            base: BaseParser::new_base_parser(
                input,
                ParserATNSimulator::new(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
            ),
            _sharedContextCache: Box::new(PredictionContextCache::new()),
            ctx: None,
            err_handler: Box::new(DefaultErrorStrategy::new()),
        }
    }
}

//pub struct CsvFileContext  {
////	base:BaseRuleContext,
//}
//
//
//impl CsvFileContext {
//    //	pub fn hdr() -> HdrContext {
////		getRuleContext(HdrContext.class,0)
////	}
////	pub fn rows() ->  Vec<RowContext>{
////		getRuleContexts(RowContext.class)
////	}
////	pub fn row(i: usize) -> RowContext{
////		getRuleContext(RowContext.class,i);
////	}
//    pub fn new(parent: Box<dyn RuleContext>, invoking_state: isize) -> Box<dyn RuleContext> {
//        Box::new(BaseRuleContext {
//            parent_ctx: Some(Box::new(parent)),
//            invoking_state,
//            rule_index: 0
//        })
//    }
////	}
//}
//
//impl<T:RuleContext> TryFrom<T> for CsvFileContext{
//    type Error = ();
//
//    fn try_from(value: T) -> Result<Self, Self::Error> {
//        unimplemented!()
//    }
//}
//
//impl CustomRuleContext for CsvFileContext{
//	fn get_rule_index(&self) -> usize { RULE_csvFile }
//}

pub trait csvFile {
    fn csvFile(&mut self) -> Result<(), ANTLRError>;
}

impl csvFile for CSVParser {
    fn csvFile(&mut self) -> Result<(), ANTLRError> {
        let mut _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        self.ctx = Some(_localctx);
        let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx, 0, RULE_csvFile);
        let mut _la = -1;
        let result: Result<(), ANTLRError> = try {
            self.base.enter_outer_alt(_localctx, 1);
            {
                self.base.set_state(8);
                self.hdr()?;
                self.base.set_state(10);
                self.err_handler.sync(&mut self.base)?;
                _la = self.base.input.la(1);
                loop {
                    {
                        {
                            self.base.set_state(9);
                            self.row()?;
                        }
                    }
                    self.base.set_state(12);
                    self.err_handler.sync(&mut self.base)?;
                    _la = self.base.input.la(1);
                    if !(((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2) | (1usize << TEXT) | (1usize << STRING))) != 0) { break; }
                }
            }
        };
        match result
            {
                Ok(_) => {}
                Err(ref re) => {
//                _localctx.exception = re;
                    self.err_handler.report_error(&mut self.base, re);
                    self.err_handler.recover(&mut self.base, re);
                }
            }
        self.base.exit_rule(&mut self.ctx);

        Ok(())
    }
}

//pub struct HdrContext  {
//	base:ParserRuleContext,
//
//
//}
//
//
//impl HdrContext{
//
//	pub fn row() -> RowContext {
//		getRuleContext(RowContext.class,0)
//	}
//	pub fn n(parent:ParserRuleContext, invokingState :usize) {
//		 Self {base:super(parent, invokingState)
// }
//	}
//	pub fn getRuleIndex() -> usize { RULE_hdr }
//}

pub trait hdr {
    fn hdr(&mut self) -> Result<(), ANTLRError>;
}

impl hdr for CSVParser {
    fn hdr(&mut self) -> Result<(), ANTLRError> {
        let _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        self.ctx = Some(_localctx);
        let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx, 2, RULE_hdr);
        {
            let result: Result<(), ANTLRError> = try {
                self.base.enter_outer_alt(_localctx, 1);
                {
                    self.base.set_state(14);
                    self.row()?;
                }
            };
            match result
                {
                    Ok(_) => {}
                    Err(ref re) => {
//				_localctx.exception = re;
                        self.err_handler.report_error(&mut self.base, re);
                        self.err_handler.recover(&mut self.base, re);
                    }
                }
        }
        self.base.exit_rule(&mut self.ctx);

        Ok(())
    }
}

//pub struct RowContext  {
//	base:ParserRuleContext,
//
//
//}
//
//
//impl RowContext{
//
//	pub fn fields() ->  Vec<FieldContext>{
//		getRuleContexts(FieldContext.class)
//	}
//	pub fn field(i: usize) -> FieldContext{
//		getRuleContext(FieldContext.class,i);
//	}
//	pub fn n(parent:ParserRuleContext, invokingState :usize) {
//		 Self {base:super(parent, invokingState)
// }
//	}
//	pub fn getRuleIndex() -> usize { RULE_row }
//}

pub trait row {
    fn row(&mut self) -> Result<(), ANTLRError>;
}

impl row for CSVParser {
    fn row(&mut self) -> Result<(), ANTLRError> {
        let _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        self.ctx = Some(_localctx);
        let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx, 4, RULE_row);
        let mut _la = -1;
        let result: Result<(), ANTLRError> = try {
            self.base.enter_outer_alt(_localctx, 1);
            {
                self.base.set_state(16);
                self.field()?;
                self.base.set_state(21);
                self.err_handler.sync(&mut self.base)?;
                _la = self.base.input.la(1);
                while _la == T__0 {
                    {
                        {
                            self.base.set_state(17);
                            self.base.match_token(T__0, self.err_handler.as_mut())?;
                            self.base.set_state(18);
                            self.field()?;
                        }
                    }
                    self.base.set_state(23);
                    self.err_handler.sync(&mut self.base)?;
                    _la = self.base.input.la(1);
                }
                self.base.set_state(25);
                self.err_handler.sync(&mut self.base)?;
                _la = self.base.input.la(1);
                if _la == T__1 {
                    {
                        self.base.set_state(24);
                        self.base.match_token(T__1, self.err_handler.as_mut())?;
                    }
                }

                self.base.set_state(27);
                self.base.match_token(T__2, self.err_handler.as_mut())?;
            }
        };
        match result
            {
                Ok(_) => {}
                Err(ref re) => {
//				_localctx.exception = re;
                    self.err_handler.report_error(&mut self.base, re);
                    self.err_handler.recover(&mut self.base, re);
                }
            }
        self.base.exit_rule(&mut self.ctx);

        Ok(())
    }
}

//pub struct FieldContext  {
//	base:ParserRuleContext,
//
//
//}
//
//
//impl FieldContext{
//
//	pub fn TEXT() -> TerminalNode { getToken(CSVParser.TEXT, 0) }
//	pub fn STRING() -> TerminalNode { getToken(CSVParser.STRING, 0) }
//	pub fn n(parent:ParserRuleContext, invokingState :usize) {
//		 Self {base:super(parent, invokingState)
// }
//	}
//	pub fn getRuleIndex() -> usize { RULE_field }
//}

pub trait field {
    fn field(&mut self) -> Result<(), ANTLRError>;
}

impl field for CSVParser {
    fn field(&mut self) -> Result<(), ANTLRError> {
        let _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        self.ctx = Some(_localctx);
        let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx, 6, RULE_field);
        let result: Result<(), ANTLRError> = try {
            self.base.set_state(32);
            self.err_handler.sync(&mut self.base);
            match self.base.input.la(1) {
                TEXT
                => {
                    self.base.enter_outer_alt(_localctx, 1);
                    {
                        self.base.set_state(29);
                        self.base.match_token(TEXT, self.err_handler.as_mut());
                    }
                }

                STRING
                => {
                    self.base.enter_outer_alt(_localctx, 2);
                    {
                        self.base.set_state(30);
                        self.base.match_token(STRING, self.err_handler.as_mut());
                    }
                }

                T__0 | T__1 | T__2
                => {
                    self.base.enter_outer_alt(_localctx, 3);
                    {}
                }

                _ =>
                    Err(ANTLRError::NoAltError(NoViableAltError::new()))?
            }
        };
        match result
            {
                Ok(_) => {}
                Err(ref re) => {
//				_localctx.exception = re;
                    self.err_handler.report_error(&mut self.base, re);
                    self.err_handler.recover(&mut self.base, re);
                }
            }
        self.base.exit_rule(&mut self.ctx);

        Ok(())
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
	\x07\x25\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x03\x02\x03\x02\x06\x02\x0d\x0a\x02\x0d\x02\x0e\x02\x0e\x03\x03\x03\x03\
	\x03\x04\x03\x04\x03\x04\x07\x04\x16\x0a\x04\x0c\x04\x0e\x04\x19\x0b\x04\
	\x03\x04\x05\x04\x1c\x0a\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x05\
	\x05\x23\x0a\x05\x03\x05\x02\x02\x06\x02\x04\x06\x08\x02\x02\x02\x25\x02\
	\x0a\x03\x02\x02\x02\x04\x10\x03\x02\x02\x02\x06\x12\x03\x02\x02\x02\x08\
	\x22\x03\x02\x02\x02\x0a\x0c\x05\x04\x03\x02\x0b\x0d\x05\x06\x04\x02\x0c\
	\x0b\x03\x02\x02\x02\x0d\x0e\x03\x02\x02\x02\x0e\x0c\x03\x02\x02\x02\x0e\
	\x0f\x03\x02\x02\x02\x0f\x03\x03\x02\x02\x02\x10\x11\x05\x06\x04\x02\x11\
	\x05\x03\x02\x02\x02\x12\x17\x05\x08\x05\x02\x13\x14\x07\x03\x02\x02\x14\
	\x16\x05\x08\x05\x02\x15\x13\x03\x02\x02\x02\x16\x19\x03\x02\x02\x02\x17\
	\x15\x03\x02\x02\x02\x17\x18\x03\x02\x02\x02\x18\x1b\x03\x02\x02\x02\x19\
	\x17\x03\x02\x02\x02\x1a\x1c\x07\x04\x02\x02\x1b\x1a\x03\x02\x02\x02\x1b\
	\x1c\x03\x02\x02\x02\x1c\x1d\x03\x02\x02\x02\x1d\x1e\x07\x05\x02\x02\x1e\
	\x07\x03\x02\x02\x02\x1f\x23\x07\x06\x02\x02\x20\x23\x07\x07\x02\x02\x21\
	\x23\x03\x02\x02\x02\x22\x1f\x03\x02\x02\x02\x22\x20\x03\x02\x02\x02\x22\
	\x21\x03\x02\x02\x02\x23\x09\x03\x02\x02\x02\x06\x0e\x17\x1b\x22";


