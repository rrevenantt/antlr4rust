// Generated from CSV.g4 by ANTLR 4.7.1
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

use super::csvlistener::*;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const WS: isize = 4;
pub const TEXT: isize = 5;
pub const STRING: isize = 6;
pub const RULE_csvFile: usize = 0;
pub const RULE_hdr: usize = 1;
pub const RULE_row: usize = 2;
pub const RULE_field: usize = 3;
pub const ruleNames: [&'static str; 4] = [
    "csvFile", "hdr", "row", "field"
];

pub const _LITERAL_NAMES: [Option<&'static str>; 4] = [
    None, Some("','"), Some("'\r'"), Some("'\n'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 7] = [
    None, None, None, None, Some("WS"), Some("TEXT"), Some("STRING")
];
lazy_static! {
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}

pub struct CSVParser {
    base: BaseParser<CSVParserExt, dyn CSVListener, CSVListenerCaller>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    err_handler: Box<dyn ErrorStrategy>,

}

impl CSVParser {
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
                CSVParserExt,
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: Box::new(DefaultErrorStrategy::new()),
        }
    }
}

impl Deref for CSVParser {
    type Target = BaseParser<CSVParserExt, dyn CSVListener, CSVListenerCaller>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CSVParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CSVParserExt;

impl ParserRecog for CSVParserExt {}

impl Recognizer for CSVParserExt {
    fn get_grammar_file_name(&self) -> &str { "CSV.g4" }

    fn get_rule_names(&self) -> &[&str] { &ruleNames }

    fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}

impl Actions for CSVParserExt {
    type Recog = dyn Parser;
}

pub struct CsvFileContext {
    base: BaseParserRuleContext<CsvFileContextExt>,

}

impl Deref for CsvFileContext {
    type Target = BaseParserRuleContext<CsvFileContextExt>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CsvFileContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CsvFileContextExt {}

impl CustomRuleContext for CsvFileContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_csvFile
    }
}


impl CsvFileContext {
    pub fn hdr(&self) -> &HdrContext {
        unimplemented!()
        //getRuleContext(HdrContext.class,0)
    }
    pub fn rows(&self) -> Vec<&RowContext> {
        unimplemented!()
        //getRuleContexts(RowContext.class)
    }
    pub fn row(&self, i: usize) -> &RowContext {
        unimplemented!()
        //getRuleContext(RowContext.class,i);
    }
    pub fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<dyn ParserRuleContext> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, CsvFileContextExt {}),
        )
    }
}


impl CSVParser {
    pub fn csvFile(&mut self) -> Result<Rc<dyn ParserRuleContext>/*CsvFileContext*/, ANTLRError> {
        let _parentctx = self.ctx.take();
        let mut _localctx = CsvFileContext::new(_parentctx.clone(), self.base.get_state());
        //let mut _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        //self.ctx = Some(_localctx);
        //let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx.clone(), 0, RULE_csvFile);
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            self.base.enter_outer_alt(None, 1);
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
                    if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2) | (1usize << TEXT) | (1usize << STRING))) != 0)) { break; }
                }
            }
        };
        match result {
            Ok(_) => {}
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

pub struct HdrContext {
    base: BaseParserRuleContext<HdrContextExt>,

}

impl Deref for HdrContext {
    type Target = BaseParserRuleContext<HdrContextExt>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for HdrContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct HdrContextExt {}

impl CustomRuleContext for HdrContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_hdr
    }
}


impl HdrContext {
    pub fn row(&self) -> &RowContext {
        unimplemented!()
        //getRuleContext(RowContext.class,0)
    }
    pub fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<dyn ParserRuleContext> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, HdrContextExt {}),
        )
    }
}


impl CSVParser {
    pub fn hdr(&mut self) -> Result<Rc<dyn ParserRuleContext>/*HdrContext*/, ANTLRError> {
        let _parentctx = self.ctx.take();
        let mut _localctx = HdrContext::new(_parentctx.clone(), self.base.get_state());
        //let mut _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        //self.ctx = Some(_localctx);
        //let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx.clone(), 2, RULE_hdr);
        let result: Result<(), ANTLRError> = try {
            self.base.enter_outer_alt(None, 1);
            {
                self.base.set_state(14);
                self.row()?;
            }
        };
        match result {
            Ok(_) => {}
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

pub struct RowContext {
    base: BaseParserRuleContext<RowContextExt>,

}

impl Deref for RowContext {
    type Target = BaseParserRuleContext<RowContextExt>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for RowContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct RowContextExt {}

impl CustomRuleContext for RowContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_row
    }
}


impl RowContext {
    pub fn fields(&self) -> Vec<&FieldContext> {
        unimplemented!()
        //getRuleContexts(FieldContext.class)
    }
    pub fn field(&self, i: usize) -> &FieldContext {
        unimplemented!()
        //getRuleContext(FieldContext.class,i);
    }
    pub fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<dyn ParserRuleContext> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, RowContextExt {}),
        )
    }
}


impl CSVParser {
    pub fn row(&mut self) -> Result<Rc<dyn ParserRuleContext>/*RowContext*/, ANTLRError> {
        let _parentctx = self.ctx.take();
        let mut _localctx = RowContext::new(_parentctx.clone(), self.base.get_state());
        //let mut _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        //self.ctx = Some(_localctx);
        //let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx.clone(), 4, RULE_row);
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            self.base.enter_outer_alt(None, 1);
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
                println!("test");
            }
        };
        match result {
            Ok(_) => {}
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

pub struct FieldContext {
    base: BaseParserRuleContext<FieldContextExt>,

}

impl Deref for FieldContext {
    type Target = BaseParserRuleContext<FieldContextExt>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for FieldContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct FieldContextExt {}

impl CustomRuleContext for FieldContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_field
    }
}


impl FieldContext {
    pub fn TEXT(&self) -> &TerminalNode {
        unimplemented!()
        //getToken(CSVParser.TEXT, 0)
    }
    pub fn STRING(&self) -> &TerminalNode {
        unimplemented!()
        //getToken(CSVParser.STRING, 0)
    }
    pub fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<dyn ParserRuleContext> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, FieldContextExt {}),
        )
    }
}


impl CSVParser {
    pub fn field(&mut self) -> Result<Rc<dyn ParserRuleContext>/*FieldContext*/, ANTLRError> {
        let _parentctx = self.ctx.take();
        let mut _localctx = FieldContext::new(_parentctx.clone(), self.base.get_state());
        //let mut _localctx = BaseRuleContext::new(self.ctx.take(), self.base.get_state());
        //self.ctx = Some(_localctx);
        //let mut _localctx = self.ctx.as_deref_mut().unwrap();
        self.base.enter_rule(_localctx.clone(), 6, RULE_field);
        let result: Result<(), ANTLRError> = try {
            self.base.set_state(33);
            self.err_handler.sync(&mut self.base)?;
            match self.base.input.la(1) {
                TEXT
                => {
                    self.base.enter_outer_alt(None, 1);
                    {
                        self.base.set_state(30);
                        self.base.match_token(TEXT, self.err_handler.as_mut())?;
                    }
                }

                STRING
                => {
                    self.base.enter_outer_alt(None, 2);
                    {
                        self.base.set_state(31);
                        self.base.match_token(STRING, self.err_handler.as_mut())?;
                    }
                }

                T__0 | T__1 | T__2
                => {
                    self.base.enter_outer_alt(None, 3);
                    {}
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new()))?
            }
        };
        match result {
            Ok(_) => {}
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
	\x08\x26\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x03\x02\x03\x02\x06\x02\x0d\x0a\x02\x0d\x02\x0e\x02\x0e\x03\x03\x03\x03\
	\x03\x04\x03\x04\x03\x04\x07\x04\x16\x0a\x04\x0c\x04\x0e\x04\x19\x0b\x04\
	\x03\x04\x05\x04\x1c\x0a\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\
	\x05\x05\x05\x24\x0a\x05\x03\x05\x02\x02\x06\x02\x04\x06\x08\x02\x02\x02\
	\x26\x02\x0a\x03\x02\x02\x02\x04\x10\x03\x02\x02\x02\x06\x12\x03\x02\x02\
	\x02\x08\x23\x03\x02\x02\x02\x0a\x0c\x05\x04\x03\x02\x0b\x0d\x05\x06\x04\
	\x02\x0c\x0b\x03\x02\x02\x02\x0d\x0e\x03\x02\x02\x02\x0e\x0c\x03\x02\x02\
	\x02\x0e\x0f\x03\x02\x02\x02\x0f\x03\x03\x02\x02\x02\x10\x11\x05\x06\x04\
	\x02\x11\x05\x03\x02\x02\x02\x12\x17\x05\x08\x05\x02\x13\x14\x07\x03\x02\
	\x02\x14\x16\x05\x08\x05\x02\x15\x13\x03\x02\x02\x02\x16\x19\x03\x02\x02\
	\x02\x17\x15\x03\x02\x02\x02\x17\x18\x03\x02\x02\x02\x18\x1b\x03\x02\x02\
	\x02\x19\x17\x03\x02\x02\x02\x1a\x1c\x07\x04\x02\x02\x1b\x1a\x03\x02\x02\
	\x02\x1b\x1c\x03\x02\x02\x02\x1c\x1d\x03\x02\x02\x02\x1d\x1e\x07\x05\x02\
	\x02\x1e\x1f\x08\x04\x01\x02\x1f\x07\x03\x02\x02\x02\x20\x24\x07\x07\x02\
	\x02\x21\x24\x07\x08\x02\x02\x22\x24\x03\x02\x02\x02\x23\x20\x03\x02\x02\
	\x02\x23\x21\x03\x02\x02\x02\x23\x22\x03\x02\x02\x02\x24\x09\x03\x02\x02\
	\x02\x06\x0e\x17\x1b\x23";


