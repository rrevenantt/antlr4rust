// Generated from CSV.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::common_token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{BaseParserRuleContext, cast, cast_mut, ParserRuleContext, ParserRuleContextType};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_source::TokenSource;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::{ParseTree, TerminalNode};
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


type BaseParserType<'input, I> = BaseParser<'input, CSVParserExt, I, dyn for<'x> CSVListener<'x>>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;

pub type LocalTokenFactory<'input> = antlr_rust::common_token_factory::ArenaCommonFactory<'input>;

pub struct CSVParser<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> {
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>,
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> CSVParser<'input, I> {
    pub fn get_serialized_atn() -> &'static str { unimplemented!() }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>>>) {
        self.err_handler = strategy
    }

    pub fn new(input: Box<I>) -> Self {
        antlr_rust::recognizer::check_version("0", "2");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                CSVParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: Box::new(DefaultErrorStrategy::<'input, LocalTokenFactory<'input>>::new()),
        }
    }
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> Deref for CSVParser<'input, I> {
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> DerefMut for CSVParser<'input, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CSVParserExt {}

impl CSVParserExt {}

//impl<'input,I: TokenStream<'input, TF=LocalTokenFactory<'input> > > ParserRecog<BaseParserType<'input,I>> for CSVParserExt{}

impl<'input> TokenAware<'input> for CSVParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input> Recognizer<'input> for CSVParserExt {
    fn get_grammar_file_name(&self) -> &str { "CSV.g4" }

    fn get_rule_names(&self) -> &[&str] { &ruleNames }

    fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> ParserRecog<'input, BaseParserType<'input, I>> for CSVParserExt {}
//------------------- csvFile ----------------
pub type CsvFileContextAll<'input> = CsvFileContext<'input>;


pub type CsvFileContext<'input> = BaseParserRuleContext<'input, CsvFileContextExt<'input>>;

#[derive(Clone)]
pub struct CsvFileContextExt<'input> {
    ph: PhantomData<&'input str>
}

impl<'input> CustomRuleContext<'input> for CsvFileContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_csvFile
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.enter_csvFile(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.exit_csvFile(ctx));
    }
}

impl<'input> CsvFileContextExt<'input> {
    fn new(parent: Option<ParserRuleContextType<'input, LocalTokenFactory<'input>>>, invoking_state: isize) -> Rc<CsvFileContextAll<'input>> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, CsvFileContextExt {
                ph: PhantomData
            }),
        )
    }
}

pub trait CsvFileContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> + BorrowMut<CsvFileContextExt<'input>> {
    fn hdr(&self) -> Option<Rc<HdrContextAll<'input>>> where Self: Sized {
        self.child_of_type(0)
    }
    fn row_all(&self) -> Vec<Rc<RowContextAll<'input>>> where Self: Sized {
        self.children_of_type()
    }
    fn row(&self, i: usize) -> Option<Rc<RowContextAll<'input>>> where Self: Sized {
        self.child_of_type(i)
    }
}

impl<'input> CsvFileContextAttrs<'input> for CsvFileContext<'input> {}

//impl CsvFileContext{

//}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> CSVParser<'input, I> {
    pub fn csvFile(&mut self)
                   -> Result<Rc<CsvFileContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = CsvFileContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_csvFile);
        let mut _localctx: Rc<CsvFileContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {

            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule hdr*/
                recog.base.set_state(8);
                recog.hdr()?;

                recog.base.set_state(10);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule row*/
                            recog.base.set_state(9);
                            recog.row()?;
                        }
                    }
                    recog.base.set_state(12);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2) | (1usize << TEXT) | (1usize << STRING))) != 0)) { break }
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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- hdr ----------------
pub type HdrContextAll<'input> = HdrContext<'input>;


pub type HdrContext<'input> = BaseParserRuleContext<'input, HdrContextExt<'input>>;

#[derive(Clone)]
pub struct HdrContextExt<'input> {
    ph: PhantomData<&'input str>
}

impl<'input> CustomRuleContext<'input> for HdrContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_hdr
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.enter_hdr(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.exit_hdr(ctx));
    }
}

impl<'input> HdrContextExt<'input> {
    fn new(parent: Option<ParserRuleContextType<'input, LocalTokenFactory<'input>>>, invoking_state: isize) -> Rc<HdrContextAll<'input>> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, HdrContextExt {
                ph: PhantomData
            }),
        )
    }
}

pub trait HdrContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> + BorrowMut<HdrContextExt<'input>> {
    fn row(&self) -> Option<Rc<RowContextAll<'input>>> where Self: Sized {
        self.child_of_type(0)
    }
}

impl<'input> HdrContextAttrs<'input> for HdrContext<'input> {}

//impl HdrContext{

//}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> CSVParser<'input, I> {
    pub fn hdr(&mut self)
               -> Result<Rc<HdrContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = HdrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_hdr);
        let mut _localctx: Rc<HdrContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {

            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule row*/
                recog.base.set_state(14);
                recog.row()?;
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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- row ----------------
pub type RowContextAll<'input> = RowContext<'input>;


pub type RowContext<'input> = BaseParserRuleContext<'input, RowContextExt<'input>>;

#[derive(Clone)]
pub struct RowContextExt<'input> {
    ph: PhantomData<&'input str>
}

impl<'input> CustomRuleContext<'input> for RowContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_row
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.enter_row(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.exit_row(ctx));
    }
}

impl<'input> RowContextExt<'input> {
    fn new(parent: Option<ParserRuleContextType<'input, LocalTokenFactory<'input>>>, invoking_state: isize) -> Rc<RowContextAll<'input>> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, RowContextExt {
                ph: PhantomData
            }),
        )
    }
}

pub trait RowContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> + BorrowMut<RowContextExt<'input>> {
    fn field_all(&self) -> Vec<Rc<FieldContextAll<'input>>> where Self: Sized {
        self.children_of_type()
    }
    fn field(&self, i: usize) -> Option<Rc<FieldContextAll<'input>>> where Self: Sized {
        self.child_of_type(i)
    }
}

impl<'input> RowContextAttrs<'input> for RowContext<'input> {}

//impl RowContext{

//}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> CSVParser<'input, I> {
    pub fn row(&mut self)
               -> Result<Rc<RowContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = RowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_row);
        let mut _localctx: Rc<RowContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {

            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule field*/
                recog.base.set_state(16);
                recog.field()?;

                recog.base.set_state(21);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__0 {
                    {
                        {
                            recog.base.set_state(17);
                            recog.base.match_token(T__0, recog.err_handler.as_mut())?;

                            /*InvokeRule field*/
                            recog.base.set_state(18);
                            recog.field()?;
                        }
                    }
                    recog.base.set_state(23);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(25);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__1 {
                    {
                        recog.base.set_state(24);
                        recog.base.match_token(T__1, recog.err_handler.as_mut())?;
                    }
                }

                recog.base.set_state(27);
                recog.base.match_token(T__2, recog.err_handler.as_mut())?;

                println!("test");
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
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- field ----------------
pub type FieldContextAll<'input> = FieldContext<'input>;


pub type FieldContext<'input> = BaseParserRuleContext<'input, FieldContextExt<'input>>;

#[derive(Clone)]
pub struct FieldContextExt<'input> {
    ph: PhantomData<&'input str>
}

impl<'input> CustomRuleContext<'input> for FieldContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_field
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.enter_field(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> CSVListener<'x>>>()
            .map(|it| it.exit_field(ctx));
    }
}

impl<'input> FieldContextExt<'input> {
    fn new(parent: Option<ParserRuleContextType<'input, LocalTokenFactory<'input>>>, invoking_state: isize) -> Rc<FieldContextAll<'input>> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, FieldContextExt {
                ph: PhantomData
            }),
        )
    }
}

pub trait FieldContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> + BorrowMut<FieldContextExt<'input>> {
    /// Retrieves first TerminalNode corresponding to token TEXT
    /// Returns `None` if there is no child corresponding to token TEXT
    fn TEXT(&self) -> Option<Rc<TerminalNode<'input, LocalTokenFactory<'input>>>> where Self: Sized {
        self.get_token(TEXT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, LocalTokenFactory<'input>>>> where Self: Sized {
        self.get_token(STRING, 0)
    }
}

impl<'input> FieldContextAttrs<'input> for FieldContext<'input> {}

//impl FieldContext{

//}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> CSVParser<'input, I> {
    pub fn field(&mut self)
                 -> Result<Rc<FieldContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_field);
        let mut _localctx: Rc<FieldContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(33);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                TEXT
                => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(30);
                        recog.base.match_token(TEXT, recog.err_handler.as_mut())?;
                    }
                }

                STRING
                => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(31);
                        recog.base.match_token(STRING, recog.err_handler.as_mut())?;
                    }
                }

                T__0 | T__1 | T__2
                => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {}
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
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
		recog.base.exit_rule();

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


