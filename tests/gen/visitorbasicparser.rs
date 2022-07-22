// Generated from VisitorBasic.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::visitorbasiclistener::*;
use super::visitorbasicvisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
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

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const A: isize = 1;
pub const RULE_s: usize = 0;
pub const ruleNames: [&'static str; 1] = ["s"];

pub const _LITERAL_NAMES: [Option<&'static str>; 2] = [None, Some("'A'")];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 2] = [None, Some("A")];
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
    VisitorBasicParserExt<'input>,
    I,
    VisitorBasicParserContextType,
    dyn VisitorBasicListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type VisitorBasicTreeWalker<'input, 'a> = ParseTreeWalker<
    'input,
    'a,
    VisitorBasicParserContextType,
    dyn VisitorBasicListener<'input> + 'a,
>;

/// Parser for VisitorBasic grammar
pub struct VisitorBasicParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> VisitorBasicParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                VisitorBasicParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> VisitorBasicParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I>
    VisitorBasicParser<'input, I, DefaultErrorStrategy<'input, VisitorBasicParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for VisitorBasicParser
pub trait VisitorBasicParserContext<'input>:
    for<'x> Listenable<dyn VisitorBasicListener<'input> + 'x>
    + for<'x> Visitable<dyn VisitorBasicVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = VisitorBasicParserContextType>
{
}

antlr_rust::coerce_from! { 'input : VisitorBasicParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn VisitorBasicParserContext<'input> + 'input
where
    T: VisitorBasicVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn VisitorBasicVisitor<'input> + 'x))
    }
}

impl<'input> VisitorBasicParserContext<'input>
    for TerminalNode<'input, VisitorBasicParserContextType>
{
}
impl<'input> VisitorBasicParserContext<'input>
    for ErrorNode<'input, VisitorBasicParserContextType>
{
}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn VisitorBasicParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn VisitorBasicListener<'input> + 'input }

pub struct VisitorBasicParserContextType;
antlr_rust::tid! {VisitorBasicParserContextType}

impl<'input> ParserNodeType<'input> for VisitorBasicParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn VisitorBasicParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for VisitorBasicParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for VisitorBasicParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct VisitorBasicParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> VisitorBasicParserExt<'input> {}
antlr_rust::tid! { VisitorBasicParserExt<'a> }

impl<'input> TokenAware<'input> for VisitorBasicParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for VisitorBasicParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for VisitorBasicParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "VisitorBasic.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- s ----------------
pub type SContextAll<'input> = SContext<'input>;

pub type SContext<'input> = BaseParserRuleContext<'input, SContextExt<'input>>;

#[derive(Clone)]
pub struct SContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> VisitorBasicParserContext<'input> for SContext<'input> {}

impl<'input, 'a> Listenable<dyn VisitorBasicListener<'input> + 'a> for SContext<'input> {
    fn enter(&self, listener: &mut (dyn VisitorBasicListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_s(self);
    }
    fn exit(&self, listener: &mut (dyn VisitorBasicListener<'input> + 'a)) {
        listener.exit_s(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn VisitorBasicVisitor<'input> + 'a> for SContext<'input> {
    fn accept(&self, visitor: &mut (dyn VisitorBasicVisitor<'input> + 'a)) {
        visitor.visit_s(self);
    }
}

impl<'input> CustomRuleContext<'input> for SContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = VisitorBasicParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_s
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_s }
}
antlr_rust::tid! {SContextExt<'a>}

impl<'input> SContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn VisitorBasicParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SContextExt { ph: PhantomData },
        ))
    }
}

pub trait SContextAttrs<'input>:
    VisitorBasicParserContext<'input> + BorrowMut<SContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<Rc<TerminalNode<'input, VisitorBasicParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, VisitorBasicParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
}

impl<'input> SContextAttrs<'input> for SContext<'input> {}

impl<'input, I, H> VisitorBasicParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn s(&mut self) -> Result<Rc<SContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_s);
        let mut _localctx: Rc<SContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(2);
                recog.base.match_token(A, &mut recog.err_handler)?;

                recog.base.set_state(3);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
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
	\x03\x08\x04\x02\x09\x02\x03\x02\x03\x02\x03\x02\x03\x02\x02\x02\x03\x02\
	\x02\x02\x02\x06\x02\x04\x03\x02\x02\x02\x04\x05\x07\x03\x02\x02\x05\x06\
	\x07\x02\x02\x03\x06\x03\x03\x02\x02\x02\x02";
