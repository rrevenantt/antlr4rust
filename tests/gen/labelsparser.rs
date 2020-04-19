// Generated from Labels.g4 by ANTLR 4.8
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

use super::labelslistener::*;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const ID: isize = 7;
pub const INT: isize = 8;
pub const WS: isize = 9;
pub const RULE_s: usize = 0;
pub const RULE_e: usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "s", "e"
];


pub const _LITERAL_NAMES: [Option<&'static str>; 7] = [
    None, Some("'*'"), Some("'+'"), Some("'('"), Some("')'"), Some("'++'"),
    Some("'--'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 10] = [
    None, None, None, None, None, None, None, Some("ID"), Some("INT"), Some("WS")
];
lazy_static! {
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> = BaseParser<'input, LabelsParserExt, I, dyn for<'x> LabelsListener<'x>>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub struct LabelsParser<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> {
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>,
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> LabelsParser<'input, I> {
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
                LabelsParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: Box::new(DefaultErrorStrategy::<'input, LocalTokenFactory<'input>>::new()),
        }
    }
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> Deref for LabelsParser<'input, I> {
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> DerefMut for LabelsParser<'input, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct LabelsParserExt {}

impl LabelsParserExt {}

//impl<'input,I: TokenStream<'input, TF=LocalTokenFactory<'input> > > ParserRecog<BaseParserType<'input,I>> for LabelsParserExt{}

impl<'input> TokenAware<'input> for LabelsParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input> Recognizer<'input> for LabelsParserExt {
    fn get_grammar_file_name(&self) -> &str { "Labels.g4" }

    fn get_rule_names(&self) -> &[&str] { &ruleNames }

    fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> ParserRecog<'input, BaseParserType<'input, I>> for LabelsParserExt {
    fn sempred(_localctx: &(dyn ParserRuleContext<'input, TF=LocalTokenFactory<'input>> + 'input), rule_index: isize, pred_index: isize,
               recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            1 => LabelsParser::<'input, I>::e_sempred(cast::<_, EContext<'input>>(_localctx), pred_index, recog),
            _ => true
        }
    }
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> LabelsParser<'input, I> {
    fn e_sempred(_localctx: &EContext<'input>, pred_index: isize,
                 recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => {
                recog.precpred(None, 7)
            }
            1 => {
                recog.precpred(None, 6)
            }
            2 => {
                recog.precpred(None, 3)
            }
            3 => {
                recog.precpred(None, 2)
            }
            _ => true
        }
	}
}
//------------------- s ----------------
pub type SContextAll<'input> = SContext<'input>;


pub type SContext<'input> = BaseParserRuleContext<'input, SContextExt<'input>>;

#[derive(Clone)]
pub struct SContextExt<'input> {
    pub q: Option<Rc<EContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for SContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_s
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_s(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_s(ctx));
    }
}

impl<'input> SContextExt<'input> {
    fn new(parent: Option<ParserRuleContextType<'input, LocalTokenFactory<'input>>>, invoking_state: isize) -> Rc<SContextAll<'input>> {
        Rc::new(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, SContextExt {
                q: None,
                ph: PhantomData,
            }),
        )
    }
}

pub trait SContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> + BorrowMut<SContextExt<'input>> {
    fn e(&self) -> Option<Rc<EContextAll<'input>>> where Self: Sized {
        self.child_of_type(0)
    }
}

impl<'input> SContextAttrs<'input> for SContext<'input> {}

//impl SContext{

//}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> LabelsParser<'input, I> {
    pub fn s(&mut self)
             -> Result<Rc<SContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_s);
        let mut _localctx: Rc<SContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {

            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule e*/
                recog.base.set_state(4);
                let tmp = recog.e_rec(0)?;
                cast_mut::<_, SContext>(&mut _localctx).q = Some(tmp.clone());
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

//------------------- e ----------------
#[derive(Debug)]
pub enum EContextAll<'input> {
    AddContext(AddContext<'input>),
    ParensContext(ParensContext<'input>),
    MultContext(MultContext<'input>),
    DecContext(DecContext<'input>),
    AnIDContext(AnIDContext<'input>),
    AnIntContext(AnIntContext<'input>),
    IncContext(IncContext<'input>),
    Error(EContext<'input>),
}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for EContextAll<'input> {}

impl<'input> Deref for EContextAll<'input> {
    type Target = dyn EContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use EContextAll::*;
        match self {
            AddContext(inner) => inner,
            ParensContext(inner) => inner,
            MultContext(inner) => inner,
            DecContext(inner) => inner,
            AnIDContext(inner) => inner,
            AnIntContext(inner) => inner,
            IncContext(inner) => inner,
            Error(inner) => inner
        }
    }
}


pub type EContext<'input> = BaseParserRuleContext<'input, EContextExt<'input>>;

#[derive(Clone)]
pub struct EContextExt<'input> {
    pub v: String,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for EContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
}

impl<'input> EContextExt<'input> {
    fn new(parent: Option<ParserRuleContextType<'input, LocalTokenFactory<'input>>>, invoking_state: isize) -> Rc<EContextAll<'input>> {
        Rc::new(
            EContextAll::Error(
                BaseParserRuleContext::new_parser_ctx(parent, invoking_state, EContextExt {
                    v: Default::default(),
                    ph: PhantomData,
                }),
            )
        )
    }
}

pub trait EContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> + BorrowMut<EContextExt<'input>> {
    fn get_v<'a>(&'a self) -> &'a String where 'input: 'a { &self.borrow().v }
    fn set_v(&mut self, attr: String) { self.borrow_mut().v = attr; }
}

impl<'input> EContextAttrs<'input> for EContext<'input> {}

//impl EContext{

//public EContext() { }
//pub fn copy_into(&self, ctx: EContext) {
//	//self.base.copyFrom(ctx);
//	self.v = ctx.v;
//}
//}

pub type AddContext<'input> = BaseParserRuleContext<'input, AddContextExt<'input>>;

pub trait AddContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> {
    fn e_all(&self) -> Vec<Rc<EContextAll<'input>>> where Self: Sized {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<Rc<EContextAll<'input>>> where Self: Sized {
        self.child_of_type(i)
    }
}

impl<'input> AddContextAttrs<'input> for AddContext<'input> {}

pub struct AddContextExt<'input> {
    base: EContextExt<'input>,
    pub a: Option<Rc<EContextAll<'input>>>,
    pub b: Option<Rc<EContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for AddContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_add(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_add(ctx));
    }
}

impl<'input> Borrow<EContextExt<'input>> for AddContext<'input> {
    fn borrow(&self) -> &EContextExt<'input> { &self.base }
}

impl<'input> BorrowMut<EContextExt<'input>> for AddContext<'input> {
    fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for AddContext<'input> {}

impl<'input> AddContextExt<'input> {
    fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>> {
        //let base = (cast::<_,EContext>(&ctx));
        Rc::new(
            EContextAll::AddContext(
                BaseParserRuleContext::copy_from(ctx, AddContextExt {
                    a: None,
                    b: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                })
            )
        )
	}
}

pub type ParensContext<'input> = BaseParserRuleContext<'input, ParensContextExt<'input>>;

pub trait ParensContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> {
    fn e(&self) -> Option<Rc<EContextAll<'input>>> where Self: Sized {
        self.child_of_type(0)
    }
}

impl<'input> ParensContextAttrs<'input> for ParensContext<'input> {}

pub struct ParensContextExt<'input> {
    base: EContextExt<'input>,
    pub x: Option<Rc<EContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for ParensContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_parens(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_parens(ctx));
    }
}

impl<'input> Borrow<EContextExt<'input>> for ParensContext<'input> {
    fn borrow(&self) -> &EContextExt<'input> { &self.base }
}

impl<'input> BorrowMut<EContextExt<'input>> for ParensContext<'input> {
    fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for ParensContext<'input> {}

impl<'input> ParensContextExt<'input> {
    fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>> {
        //let base = (cast::<_,EContext>(&ctx));
        Rc::new(
            EContextAll::ParensContext(
                BaseParserRuleContext::copy_from(ctx, ParensContextExt {
                    x: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                })
            )
        )
	}
}

pub type MultContext<'input> = BaseParserRuleContext<'input, MultContextExt<'input>>;

pub trait MultContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> {
    fn e_all(&self) -> Vec<Rc<EContextAll<'input>>> where Self: Sized {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<Rc<EContextAll<'input>>> where Self: Sized {
        self.child_of_type(i)
    }
}

impl<'input> MultContextAttrs<'input> for MultContext<'input> {}

pub struct MultContextExt<'input> {
    base: EContextExt<'input>,
    pub a: Option<Rc<EContextAll<'input>>>,
    pub op: Option<TokenType<'input>>,
    pub b: Option<Rc<EContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for MultContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_mult(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_mult(ctx));
    }
}

impl<'input> Borrow<EContextExt<'input>> for MultContext<'input> {
    fn borrow(&self) -> &EContextExt<'input> { &self.base }
}

impl<'input> BorrowMut<EContextExt<'input>> for MultContext<'input> {
    fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for MultContext<'input> {}

impl<'input> MultContextExt<'input> {
    fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>> {
        //let base = (cast::<_,EContext>(&ctx));
        Rc::new(
            EContextAll::MultContext(
                BaseParserRuleContext::copy_from(ctx, MultContextExt {
                    op: None,
                    a: None,
                    b: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                })
            )
		)
	}
}

pub type DecContext<'input> = BaseParserRuleContext<'input, DecContextExt<'input>>;

pub trait DecContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> {
    fn e(&self) -> Option<Rc<EContextAll<'input>>> where Self: Sized {
        self.child_of_type(0)
    }
}

impl<'input> DecContextAttrs<'input> for DecContext<'input> {}

pub struct DecContextExt<'input> {
    base: EContextExt<'input>,
    pub x: Option<Rc<EContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for DecContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_dec(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_dec(ctx));
    }
}

impl<'input> Borrow<EContextExt<'input>> for DecContext<'input> {
    fn borrow(&self) -> &EContextExt<'input> { &self.base }
}

impl<'input> BorrowMut<EContextExt<'input>> for DecContext<'input> {
    fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for DecContext<'input> {}

impl<'input> DecContextExt<'input> {
    fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>> {
        //let base = (cast::<_,EContext>(&ctx));
        Rc::new(
            EContextAll::DecContext(
                BaseParserRuleContext::copy_from(ctx, DecContextExt {
                    x: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                })
            )
        )
	}
}

pub type AnIDContext<'input> = BaseParserRuleContext<'input, AnIDContextExt<'input>>;

pub trait AnIDContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> {
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, LocalTokenFactory<'input>>>> where Self: Sized {
        self.get_token(ID, 0)
    }
}

impl<'input> AnIDContextAttrs<'input> for AnIDContext<'input> {}

pub struct AnIDContextExt<'input> {
    base: EContextExt<'input>,
    pub ID: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for AnIDContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_anID(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_anID(ctx));
    }
}

impl<'input> Borrow<EContextExt<'input>> for AnIDContext<'input> {
    fn borrow(&self) -> &EContextExt<'input> { &self.base }
}

impl<'input> BorrowMut<EContextExt<'input>> for AnIDContext<'input> {
    fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for AnIDContext<'input> {}

impl<'input> AnIDContextExt<'input> {
    fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>> {
        //let base = (cast::<_,EContext>(&ctx));
        Rc::new(
            EContextAll::AnIDContext(
                BaseParserRuleContext::copy_from(ctx, AnIDContextExt {
                    ID: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                })
            )
        )
	}
}

pub type AnIntContext<'input> = BaseParserRuleContext<'input, AnIntContextExt<'input>>;

pub trait AnIntContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> {
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<Rc<TerminalNode<'input, LocalTokenFactory<'input>>>> where Self: Sized {
        self.get_token(INT, 0)
    }
}

impl<'input> AnIntContextAttrs<'input> for AnIntContext<'input> {}

pub struct AnIntContextExt<'input> {
    base: EContextExt<'input>,
    pub INT: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for AnIntContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_anInt(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_anInt(ctx));
    }
}

impl<'input> Borrow<EContextExt<'input>> for AnIntContext<'input> {
    fn borrow(&self) -> &EContextExt<'input> { &self.base }
}

impl<'input> BorrowMut<EContextExt<'input>> for AnIntContext<'input> {
    fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for AnIntContext<'input> {}

impl<'input> AnIntContextExt<'input> {
    fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>> {
        //let base = (cast::<_,EContext>(&ctx));
        Rc::new(
            EContextAll::AnIntContext(
                BaseParserRuleContext::copy_from(ctx, AnIntContextExt {
                    INT: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                })
            )
        )
	}
}

pub type IncContext<'input> = BaseParserRuleContext<'input, IncContextExt<'input>>;

pub trait IncContextAttrs<'input>: ParserRuleContext<'input, TF=LocalTokenFactory<'input>> {
    fn e(&self) -> Option<Rc<EContextAll<'input>>> where Self: Sized {
        self.child_of_type(0)
    }
}

impl<'input> IncContextAttrs<'input> for IncContext<'input> {}

pub struct IncContextExt<'input> {
    base: EContextExt<'input>,
    pub x: Option<Rc<EContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> CustomRuleContext<'input> for IncContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    fn get_rule_index(&self) -> usize {
        RULE_e
    }
    fn enter(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.enter_inc(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<'input, Self>, listener: &mut dyn Any) where Self: Sized {
        listener.downcast_mut::<Box<dyn for<'x> LabelsListener<'x>>>()
            .map(|it| it.exit_inc(ctx));
    }
}

impl<'input> Borrow<EContextExt<'input>> for IncContext<'input> {
    fn borrow(&self) -> &EContextExt<'input> { &self.base }
}

impl<'input> BorrowMut<EContextExt<'input>> for IncContext<'input> {
    fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for IncContext<'input> {}

impl<'input> IncContextExt<'input> {
    fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>> {
        //let base = (cast::<_,EContext>(&ctx));
        Rc::new(
            EContextAll::IncContext(
                BaseParserRuleContext::copy_from(ctx, IncContextExt {
                    x: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                })
            )
        )
	}
}

impl<'input, I: TokenStream<'input, TF=LocalTokenFactory<'input>>> LabelsParser<'input, I> {
    pub fn e(&mut self)
             -> Result<Rc<EContextAll<'input>>, ANTLRError> {
        self.e_rec(0)
    }

    fn e_rec(&mut self, _p: isize)
             -> Result<Rc<EContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = EContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_recursion_rule(_localctx.clone(), 2, RULE_e, _p);
        let mut _localctx: Rc<EContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 2;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(16);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    INT
                    => {
                        {
                            let mut tmp = AnIntContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();

                            recog.base.set_state(7);
                            let tmp = recog.base.match_token(INT, recog.err_handler.as_mut())?;
                            if let EContextAll::AnIntContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                ctx.INT = Some(tmp.clone());
                            } else { unreachable!("cant cast"); }

                            let tmp = {
                                if let Some(it) = &if let EContextAll::AnIntContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                    ctx
                                } else { unreachable!("cant cast") }.INT { it.get_text() } else { "null" }.to_owned()
                            }.to_owned();
                            if let EContextAll::AnIntContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                ctx.set_v(tmp);
                            } else { unreachable!("cant cast"); }
                        }
                    }

                    T__2
                    => {
                        {
                            let mut tmp = ParensContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(9);
                            recog.base.match_token(T__2, recog.err_handler.as_mut())?;

                            /*InvokeRule e*/
                            recog.base.set_state(10);
                            let tmp = recog.e_rec(0)?;
                            if let EContextAll::ParensContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                ctx.x = Some(tmp.clone());
                            } else { unreachable!("cant cast"); }

                            recog.base.set_state(11);
                            recog.base.match_token(T__3, recog.err_handler.as_mut())?;

                            let tmp = {
                                if let EContextAll::ParensContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                    ctx
                                } else { unreachable!("cant cast") }.x.as_ref().unwrap().get_v()
                            }.to_owned();
                            if let EContextAll::ParensContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                ctx.set_v(tmp);
                            } else { unreachable!("cant cast"); }
                        }
                    }

                    ID
                    => {
                        {
                            let mut tmp = AnIDContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(14);
                            let tmp = recog.base.match_token(ID, recog.err_handler.as_mut())?;
                            if let EContextAll::AnIDContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                ctx.ID = Some(tmp.clone());
                            } else { unreachable!("cant cast"); }

                            let tmp = {
                                if let Some(it) = &if let EContextAll::AnIDContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                    ctx
                                } else { unreachable!("cant cast") }.ID { it.get_text() } else { "null" }.to_owned()
                            }.to_owned();
                            if let EContextAll::AnIDContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                ctx.set_v(tmp);
                            } else { unreachable!("cant cast"); }
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
                }
                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(36);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(2, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(34);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(1, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = MultContextExt::new(&**EContextExt::new(_parentctx.clone(), _parentState));
                                        if let EContextAll::MultContext(ctx) = cast_mut::<_, EContextAll>(&mut tmp) {
                                            ctx.a = Some(_prevctx.clone());
                                        } else { unreachable!("cant cast"); }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
                                        _localctx = tmp;
                                        recog.base.set_state(18);
                                        if !({ recog.precpred(None, 7) }) {
                                            Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
                                        }
                                        recog.base.set_state(19);
                                        let tmp = recog.base.match_token(T__0, recog.err_handler.as_mut())?;
                                        if let EContextAll::MultContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                            ctx.op = Some(tmp.clone());
                                        } else { unreachable!("cant cast"); }

                                        /*InvokeRule e*/
                                        recog.base.set_state(20);
                                        let tmp = recog.e_rec(8)?;
                                        if let EContextAll::MultContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                            ctx.b = Some(tmp.clone());
                                        } else { unreachable!("cant cast"); }

                                        let tmp = {
                                            "* ".to_owned() + if let EContextAll::MultContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                                ctx
                                            } else { unreachable!("cant cast") }.a.as_ref().unwrap().get_v() + " " + if let EContextAll::MultContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                                ctx
                                            } else { unreachable!("cant cast") }.b.as_ref().unwrap().get_v()
                                        }.to_owned();
                                        if let EContextAll::MultContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                            ctx.set_v(tmp);
                                        } else { unreachable!("cant cast"); }
                                    }
                                }
                                ,
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = AddContextExt::new(&**EContextExt::new(_parentctx.clone(), _parentState));
                                        if let EContextAll::AddContext(ctx) = cast_mut::<_, EContextAll>(&mut tmp) {
                                            ctx.a = Some(_prevctx.clone());
                                        } else { unreachable!("cant cast"); }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
                                        _localctx = tmp;
                                        recog.base.set_state(23);
                                        if !({ recog.precpred(None, 6) }) {
                                            Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
                                        }
                                        recog.base.set_state(24);
                                        recog.base.match_token(T__1, recog.err_handler.as_mut())?;

                                        /*InvokeRule e*/
                                        recog.base.set_state(25);
                                        let tmp = recog.e_rec(7)?;
                                        if let EContextAll::AddContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                            ctx.b = Some(tmp.clone());
                                        } else { unreachable!("cant cast"); }

                                        let tmp = {
                                            "+ ".to_owned() + if let EContextAll::AddContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                                ctx
                                            } else { unreachable!("cant cast") }.a.as_ref().unwrap().get_v() + " " + if let EContextAll::AddContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                                ctx
                                            } else { unreachable!("cant cast") }.b.as_ref().unwrap().get_v()
                                        }.to_owned();
                                        if let EContextAll::AddContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                            ctx.set_v(tmp);
                                        } else { unreachable!("cant cast"); }
                                    }
                                }
                                ,
                                3 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = IncContextExt::new(&**EContextExt::new(_parentctx.clone(), _parentState));
                                        if let EContextAll::IncContext(ctx) = cast_mut::<_, EContextAll>(&mut tmp) {
                                            ctx.x = Some(_prevctx.clone());
                                        } else { unreachable!("cant cast"); }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
                                        _localctx = tmp;
                                        recog.base.set_state(28);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
                                        }
                                        recog.base.set_state(29);
                                        recog.base.match_token(T__4, recog.err_handler.as_mut())?;

                                        let tmp = {
                                            " ++".to_owned() + if let EContextAll::IncContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                                ctx
                                            } else { unreachable!("cant cast") }.x.as_ref().unwrap().get_v()
                                        }.to_owned();
                                        if let EContextAll::IncContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                            ctx.set_v(tmp);
                                        } else { unreachable!("cant cast"); }
                                    }
                                }
                                ,
                                4 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = DecContextExt::new(&**EContextExt::new(_parentctx.clone(), _parentState));
                                        if let EContextAll::DecContext(ctx) = cast_mut::<_, EContextAll>(&mut tmp) {
                                            ctx.x = Some(_prevctx.clone());
                                        } else { unreachable!("cant cast"); }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
                                        _localctx = tmp;
                                        recog.base.set_state(31);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
                                        }
                                        recog.base.set_state(32);
                                        recog.base.match_token(T__5, recog.err_handler.as_mut())?;

                                        let tmp = {
                                            " --".to_owned() + if let EContextAll::DecContext(ctx) = cast::<_, EContextAll>(&*_localctx) {
                                                ctx
                                            } else { unreachable!("cant cast") }.x.as_ref().unwrap().get_v()
                                        }.to_owned();
                                        if let EContextAll::DecContext(ctx) = cast_mut::<_, EContextAll>(&mut _localctx) {
                                            ctx.set_v(tmp);
                                        } else { unreachable!("cant cast"); }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(38);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(2, &mut recog.base)?;
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
	\x0b\x2a\x04\x02\x09\x02\x04\x03\x09\x03\x03\x02\x03\x02\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\
	\x13\x0a\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x07\
	\x03\x25\x0a\x03\x0c\x03\x0e\x03\x28\x0b\x03\x03\x03\x02\x03\x04\x04\x02\
	\x04\x02\x02\x02\x2d\x02\x06\x03\x02\x02\x02\x04\x12\x03\x02\x02\x02\x06\
	\x07\x05\x04\x03\x02\x07\x03\x03\x02\x02\x02\x08\x09\x08\x03\x01\x02\x09\
	\x0a\x07\x0a\x02\x02\x0a\x13\x08\x03\x01\x02\x0b\x0c\x07\x05\x02\x02\x0c\
	\x0d\x05\x04\x03\x02\x0d\x0e\x07\x06\x02\x02\x0e\x0f\x08\x03\x01\x02\x0f\
	\x13\x03\x02\x02\x02\x10\x11\x07\x09\x02\x02\x11\x13\x08\x03\x01\x02\x12\
	\x08\x03\x02\x02\x02\x12\x0b\x03\x02\x02\x02\x12\x10\x03\x02\x02\x02\x13\
	\x26\x03\x02\x02\x02\x14\x15\x0c\x09\x02\x02\x15\x16\x07\x03\x02\x02\x16\
	\x17\x05\x04\x03\x0a\x17\x18\x08\x03\x01\x02\x18\x25\x03\x02\x02\x02\x19\
	\x1a\x0c\x08\x02\x02\x1a\x1b\x07\x04\x02\x02\x1b\x1c\x05\x04\x03\x09\x1c\
	\x1d\x08\x03\x01\x02\x1d\x25\x03\x02\x02\x02\x1e\x1f\x0c\x05\x02\x02\x1f\
	\x20\x07\x07\x02\x02\x20\x25\x08\x03\x01\x02\x21\x22\x0c\x04\x02\x02\x22\
	\x23\x07\x08\x02\x02\x23\x25\x08\x03\x01\x02\x24\x14\x03\x02\x02\x02\x24\
	\x19\x03\x02\x02\x02\x24\x1e\x03\x02\x02\x02\x24\x21\x03\x02\x02\x02\x25\
	\x28\x03\x02\x02\x02\x26\x24\x03\x02\x02\x02\x26\x27\x03\x02\x02\x02\x27\
	\x05\x03\x02\x02\x02\x28\x26\x03\x02\x02\x02\x05\x12\x24\x26";


