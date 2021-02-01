// Generated from ReferenceToATN.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::referencetoatnlistener::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::lazy_static;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const ID: isize = 1;
pub const ATN: isize = 2;
pub const WS: isize = 3;
pub const RULE_a: usize = 0;
pub const ruleNames: [&'static str; 1] = ["a"];

pub const _LITERAL_NAMES: [Option<&'static str>; 0] = [];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 4] = [None, Some("ID"), Some("ATN"), Some("WS")];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    ReferenceToATNParserExt,
    I,
    ReferenceToATNParserContextType,
    dyn ReferenceToATNListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;

pub type LocalTokenFactory<'input> = antlr_rust::token_factory::OwningTokenFactory;

pub type ReferenceToATNTreeWalker<'input, 'a> = ParseTreeWalker<
    'input,
    'a,
    ReferenceToATNParserContextType,
    dyn ReferenceToATNListener<'input> + 'a,
>;

/// Parser for ReferenceToATN grammar
pub struct ReferenceToATNParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> ReferenceToATNParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) { self.err_handler = strategy }

    pub fn with_strategy(input: I, strategy: H) -> Self {
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
                ReferenceToATNParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> ReferenceToATNParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I>
    ReferenceToATNParser<'input, I, DefaultErrorStrategy<'input, ReferenceToATNParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self { Self::with_strategy(input, DefaultErrorStrategy::new()) }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ReferenceToATNParser
pub trait ReferenceToATNParserContext<'input>: for<'x> Listenable<dyn ReferenceToATNListener<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = ReferenceToATNParserContextType>
{
}

impl<'input> ReferenceToATNParserContext<'input>
    for TerminalNode<'input, ReferenceToATNParserContextType>
{
}
impl<'input> ReferenceToATNParserContext<'input>
    for ErrorNode<'input, ReferenceToATNParserContextType>
{
}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn ReferenceToATNParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn ReferenceToATNListener<'input> + 'input {}

pub struct ReferenceToATNParserContextType;
antlr_rust::type_id! {ReferenceToATNParserContextType}

impl<'input> ParserNodeType<'input> for ReferenceToATNParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn ReferenceToATNParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for ReferenceToATNParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target { &self.base }
}

impl<'input, I, H> DerefMut for ReferenceToATNParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub struct ReferenceToATNParserExt {}

impl ReferenceToATNParserExt {}

impl<'input> TokenAware<'input> for ReferenceToATNParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for ReferenceToATNParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for ReferenceToATNParserExt
{
    fn get_grammar_file_name(&self) -> &str { "ReferenceToATN.g4" }

    fn get_rule_names(&self) -> &[&str] { &ruleNames }

    fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- a ----------------
pub type AContextAll<'input> = AContext<'input>;

pub type AContext<'input> = BaseParserRuleContext<'input, AContextExt<'input>>;

#[derive(Clone)]
pub struct AContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> ReferenceToATNParserContext<'input> for AContext<'input> {}

impl<'input, 'a> Listenable<dyn ReferenceToATNListener<'input> + 'a> for AContext<'input> {
    fn enter(&self, listener: &mut (dyn ReferenceToATNListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_a(self);
    }
}

impl<'input> CustomRuleContext<'input> for AContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = ReferenceToATNParserContextType;
    fn get_rule_index(&self) -> usize { RULE_a }
    //fn type_rule_index() -> usize where Self: Sized { RULE_a }
}
antlr_rust::type_id! {AContextExt<'a>}

impl<'input> AContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn ReferenceToATNParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AContextExt { ph: PhantomData },
        ))
    }
}

pub trait AContextAttrs<'input>:
    ReferenceToATNParserContext<'input> + BorrowMut<AContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token ATN in current rule
    fn ATN_all(&self) -> Vec<Rc<TerminalNode<'input, ReferenceToATNParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ATN, starting from 0.
    /// Returns `None` if number of children corresponding to token ATN is less or equal than `i`.
    fn ATN(&self, i: usize) -> Option<Rc<TerminalNode<'input, ReferenceToATNParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ATN, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<Rc<TerminalNode<'input, ReferenceToATNParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
    fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input, ReferenceToATNParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, i)
    }
}

impl<'input> AContextAttrs<'input> for AContext<'input> {}

impl<'input, I, H> ReferenceToATNParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn a(&mut self) -> Result<Rc<AContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_a);
        let mut _localctx: Rc<AContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(5);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(2);
                                _la = recog.base.input.la(1);
                                if { !(_la == ID || _la == ATN) } {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                            }
                        }
                    }
                    recog.base.set_state(7);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                }
                recog.base.set_state(9);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ATN {
                    {
                        recog.base.set_state(8);
                        recog.base.match_token(ATN, &mut recog.err_handler)?;
                    }
                }

                println!("{}", {
                    let temp = recog
                        .base
                        .input
                        .lt(-1)
                        .map(|it| it.get_token_index())
                        .unwrap_or(-1);
                    recog.input.get_text_from_interval(
                        recog.get_parser_rule_context().start().get_token_index(),
                        temp,
                    )
                });
            }
        };
        match result {
            Ok(_) => {}
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
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
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
