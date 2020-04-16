//use tree::RuleNode;

use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use crate::atn::INVALID_ALT;
use crate::common_token_factory::{CommonTokenFactory, TokenFactory};
use crate::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, ParserRuleContextType};

//pub trait RuleContext:RuleNode {
pub trait RuleContext<'input>: CustomRuleContext<'input> {
    fn get_invoking_state(&self) -> isize;
    fn set_invoking_state(&self, t: isize);

    /// A context is empty if there is no invoking state; meaning nobody called
    /// current context. Which is usually true for the root of the syntax tree
    fn is_empty(&self) -> bool {
        self.get_invoking_state() == -1
    }

    fn get_parent_ctx(&self) -> Option<ParserRuleContextType<'input, Self::TF>>;

    fn set_parent(&self, parent: &Option<ParserRuleContextType<'input, Self::TF>>);
}

pub struct EmptyCustomRuleContext<'a, TF: TokenFactory<'a> + 'a>(pub(crate) PhantomData<&'a TF::Tok>);

impl<'a, TF: TokenFactory<'a> + 'a> CustomRuleContext<'a> for EmptyCustomRuleContext<'a, TF> {
    type TF = TF;

    fn get_rule_index(&self) -> usize {
        usize::max_value()
    }
}

pub trait CustomRuleContext<'input> {
    type TF: TokenFactory<'input> + 'input;
    //const RULE_INDEX:usize;
    fn get_rule_index(&self) -> usize;
    fn type_rule_index() -> usize where Self: Sized { unimplemented!() }

    fn get_alt_number(&self) -> isize { INVALID_ALT }
    fn set_alt_number(&self, _alt_number: isize) {}
    fn enter(_ctx: &BaseParserRuleContext<'input, Self>, _listener: &mut dyn Any) where Self: Sized {}
    fn exit(_ctx: &BaseParserRuleContext<'input, Self>, _listener: &mut dyn Any) where Self: Sized {}
}

pub struct BaseRuleContext<'input, ExtCtx: CustomRuleContext<'input>> {
    pub(crate) parent_ctx: RefCell<Option<Weak<dyn ParserRuleContext<'input, TF=ExtCtx::TF> + 'input>>>,
    invoking_state: Cell<isize>,
    pub(crate) ext: ExtCtx,
}

impl<'input, ExtCtx: CustomRuleContext<'input>> BaseRuleContext<'input, ExtCtx> {
    pub(crate) fn new_ctx(parent_ctx: Option<ParserRuleContextType<'input, ExtCtx::TF>>, invoking_state: isize, ext: ExtCtx) -> Self {
        BaseRuleContext {
            parent_ctx: RefCell::new(parent_ctx.as_ref().map(Rc::downgrade)),
            invoking_state: Cell::new(invoking_state),
            ext,
        }
    }
}

impl<'input, ExtCtx: CustomRuleContext<'input>> CustomRuleContext<'input> for BaseRuleContext<'input, ExtCtx> {
    type TF = ExtCtx::TF;

    fn get_rule_index(&self) -> usize {
        unimplemented!()
    }
}

impl<'input, ExtCtx: CustomRuleContext<'input>> RuleContext<'input> for BaseRuleContext<'input, ExtCtx> {
    fn get_invoking_state(&self) -> isize {
        self.invoking_state.get()
    }

    fn set_invoking_state(&self, t: isize) {
        self.invoking_state.set(t)
    }

    fn get_parent_ctx(&self) -> Option<ParserRuleContextType<'input, Self::TF>> {
        self.parent_ctx.borrow().as_ref().map(Weak::upgrade).flatten()
    }

//    fn get_parent_ctx(&self) -> Option<ParserRuleContextType> {
//        self.parent_ctx.borrow().as_ref().map(Weak::upgrade).map(Option::unwrap)
//    }

    fn set_parent(&self, parent: &Option<ParserRuleContextType<'input, Self::TF>>) {
        *self.parent_ctx.borrow_mut() = parent.as_ref().map(Rc::downgrade);
    }
}
