//use tree::RuleNode;

use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::fmt::{Debug, Formatter};
use std::iter::from_fn;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use crate::atn::INVALID_ALT;
use crate::parser::ParserNodeType;
use crate::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use crate::token_factory::{CommonTokenFactory, TokenFactory};
use crate::tree::{ParseTree, ParseTreeListener, Tree};

//pub trait RuleContext:RuleNode {
pub trait RuleContext<'input>: CustomRuleContext<'input> {
    fn get_invoking_state(&self) -> isize { -1 }
    fn set_invoking_state(&self, t: isize) {}

    /// A context is empty if there is no invoking state; meaning nobody called
    /// current context. Which is usually true for the root of the syntax tree
    fn is_empty(&self) -> bool { self.get_invoking_state() == -1 }

    fn get_parent_ctx(&self) -> Option<Rc<<Self::Ctx as ParserNodeType<'input>>::Type>> { None }

    fn set_parent(&self, parent: &Option<Rc<<Self::Ctx as ParserNodeType<'input>>::Type>>) {}
}

pub(crate) fn states_stack<'input, T: ParserRuleContext<'input> + ?Sized + 'input>(
    mut ctx: Rc<T>,
) -> impl Iterator<Item = isize>
where
    T::Ctx: ParserNodeType<'input, Type = T>,
{
    from_fn(move || {
        if ctx.get_invoking_state() < 0 {
            None
        } else {
            let state = ctx.get_invoking_state();
            ctx = ctx.get_parent_ctx().unwrap();
            Some(state)
        }
    })
}

#[doc(hidden)]
pub unsafe trait Tid {
    fn self_id(&self) -> TypeId;
    fn id() -> TypeId
    where
        Self: Sized;
}

pub struct EmptyCustomRuleContext<'a, TF: TokenFactory<'a> + 'a>(
    pub(crate) PhantomData<&'a TF::Tok>,
);

impl<'a, TF: TokenFactory<'a> + 'a> CustomRuleContext<'a> for EmptyCustomRuleContext<'a, TF> {
    type TF = TF;
    type Ctx = EmptyContextType<'a, TF>;

    fn get_rule_index(&self) -> usize { usize::max_value() }
}

unsafe impl<'a, TF: TokenFactory<'a> + 'a> Tid for EmptyCustomRuleContext<'a, TF> {
    fn self_id(&self) -> TypeId {
        TypeId::of::<EmptyCustomRuleContext<'static, CommonTokenFactory>>()
    }

    fn id() -> TypeId
    where
        Self: Sized,
    {
        TypeId::of::<EmptyCustomRuleContext<'static, CommonTokenFactory>>()
    }
}

pub type EmptyContext<'a, TF> =
    dyn ParserRuleContext<'a, TF = TF, Ctx = EmptyContextType<'a, TF>> + 'a;

pub struct EmptyContextType<'a, TF: TokenFactory<'a>>(pub PhantomData<&'a TF>);

impl<'a, TF: TokenFactory<'a>> ParserNodeType<'a> for EmptyContextType<'a, TF> {
    type TF = TF;
    type Type = dyn ParserRuleContext<'a, TF = Self::TF, Ctx = Self> + 'a;
}

pub trait CustomRuleContext<'input>: Tid {
    type TF: TokenFactory<'input> + 'input;
    type Ctx: ParserNodeType<'input, TF = Self::TF>;
    //const RULE_INDEX:usize;
    fn get_rule_index(&self) -> usize;

    fn get_alt_number(&self) -> isize { INVALID_ALT }
    fn set_alt_number(&self, _alt_number: isize) {}
    // fn enter(_ctx: &dyn Tree<'input, Node=Self>, _listener: &mut dyn Any) where Self: Sized {}
    // fn exit(_ctx: &dyn Tree<'input, Node=Self>, _listener: &mut dyn Any) where Self: Sized {}
}

pub struct BaseRuleContext<'input, ExtCtx: CustomRuleContext<'input>> {
    pub(crate) parent_ctx: RefCell<Option<Weak<<ExtCtx::Ctx as ParserNodeType<'input>>::Type>>>,
    invoking_state: Cell<isize>,
    pub(crate) ext: ExtCtx,
}

impl<'input, ExtCtx: CustomRuleContext<'input>> BaseRuleContext<'input, ExtCtx> {
    pub(crate) fn new_ctx(
        parent_ctx: Option<Rc<<ExtCtx::Ctx as ParserNodeType<'input>>::Type>>,
        invoking_state: isize,
        ext: ExtCtx,
    ) -> Self {
        BaseRuleContext {
            parent_ctx: RefCell::new(parent_ctx.as_ref().map(Rc::downgrade)),
            invoking_state: Cell::new(invoking_state),
            ext,
        }
    }
}

impl<'input, ExtCtx: CustomRuleContext<'input>> CustomRuleContext<'input>
    for BaseRuleContext<'input, ExtCtx>
{
    type TF = ExtCtx::TF;
    type Ctx = ExtCtx::Ctx;

    fn get_rule_index(&self) -> usize { self.ext.get_rule_index() }
}

unsafe impl<'input, Ctx: CustomRuleContext<'input>> Tid for BaseRuleContext<'input, Ctx> {
    fn self_id(&self) -> TypeId { self.ext.self_id() }

    fn id() -> TypeId
    where
        Self: Sized,
    {
        Ctx::id()
    }
}

impl<'input, ExtCtx: CustomRuleContext<'input>> RuleContext<'input>
    for BaseRuleContext<'input, ExtCtx>
{
    fn get_invoking_state(&self) -> isize { self.invoking_state.get() }

    fn set_invoking_state(&self, t: isize) { self.invoking_state.set(t) }

    fn get_parent_ctx(&self) -> Option<Rc<<ExtCtx::Ctx as ParserNodeType<'input>>::Type>> {
        self.parent_ctx
            .borrow()
            .as_ref()
            .map(Weak::upgrade)
            .flatten()
    }

    //    fn get_parent_ctx(&self) -> Option<ParserRuleContextType> {
    //        self.parent_ctx.borrow().as_ref().map(Weak::upgrade).map(Option::unwrap)
    //    }

    fn set_parent(&self, parent: &Option<Rc<<ExtCtx::Ctx as ParserNodeType<'input>>::Type>>) {
        *self.parent_ctx.borrow_mut() = parent.as_ref().map(Rc::downgrade);
    }
}

impl<'input, ExtCtx: CustomRuleContext<'input>> Debug for BaseRuleContext<'input, ExtCtx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { unimplemented!() }
}

impl<'input, ExtCtx: CustomRuleContext<'input>> Tree<'input> for BaseRuleContext<'input, ExtCtx> {}

impl<'input, ExtCtx: CustomRuleContext<'input>> ParseTree<'input>
    for BaseRuleContext<'input, ExtCtx>
{
}

impl<'input, ExtCtx: CustomRuleContext<'input>> ParserRuleContext<'input>
    for BaseRuleContext<'input, ExtCtx>
{
}
