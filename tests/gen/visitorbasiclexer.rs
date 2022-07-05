// Generated from VisitorBasic.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::char_stream::CharStream;
use antlr_rust::dfa::DFA;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use antlr_rust::parser_rule_context::{cast, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, EmptyContext, EmptyCustomRuleContext};
use antlr_rust::token::*;
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::{lazy_static, Tid, TidAble, TidExt};

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const A: isize = 1;
pub const channelNames: [&'static str; 0 + 2] = ["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];

pub const modeNames: [&'static str; 1] = ["DEFAULT_MODE"];

pub const ruleNames: [&'static str; 1] = ["A"];

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

pub type LexerContext<'input> =
    BaseRuleContext<'input, EmptyCustomRuleContext<'input, LocalTokenFactory<'input>>>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a>>::From;

pub struct VisitorBasicLexer<'input, Input: CharStream<From<'input>>> {
    base: BaseLexer<'input, VisitorBasicLexerActions, Input, LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for VisitorBasicLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input: CharStream<From<'input>>> Deref for VisitorBasicLexer<'input, Input> {
    type Target = BaseLexer<'input, VisitorBasicLexerActions, Input, LocalTokenFactory<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> DerefMut for VisitorBasicLexer<'input, Input> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> VisitorBasicLexer<'input, Input> {
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
        "VisitorBasicLexer.g4"
    }

    pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
                VisitorBasicLexerActions {},
                tf,
            ),
        }
    }
}

impl<'input, Input: CharStream<From<'input>>> VisitorBasicLexer<'input, Input>
where
    &'input LocalTokenFactory<'input>: Default,
{
    pub fn new(input: Input) -> Self {
        VisitorBasicLexer::new_with_token_factory(
            input,
            <&LocalTokenFactory<'input> as Default>::default(),
        )
    }
}

pub struct VisitorBasicLexerActions {}

impl VisitorBasicLexerActions {}

impl<'input, Input: CharStream<From<'input>>>
    Actions<'input, BaseLexer<'input, VisitorBasicLexerActions, Input, LocalTokenFactory<'input>>>
    for VisitorBasicLexerActions
{
}

impl<'input, Input: CharStream<From<'input>>> VisitorBasicLexer<'input, Input> {}

impl<'input, Input: CharStream<From<'input>>>
    LexerRecog<
        'input,
        BaseLexer<'input, VisitorBasicLexerActions, Input, LocalTokenFactory<'input>>,
    > for VisitorBasicLexerActions
{
}
impl<'input> TokenAware<'input> for VisitorBasicLexerActions {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, Input: CharStream<From<'input>>> TokenSource<'input>
    for VisitorBasicLexer<'input, Input>
{
    type TF = LocalTokenFactory<'input>;

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
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x03\x07\x08\x01\x04\x02\x09\x02\x03\x02\x03\x02\x02\x02\x03\x03\x03\x03\
		\x02\x02\x02\x06\x02\x03\x03\x02\x02\x02\x03\x05\x03\x02\x02\x02\x05\x06\
		\x07\x43\x02\x02\x06\x04\x03\x02\x02\x02\x03\x02\x02";
