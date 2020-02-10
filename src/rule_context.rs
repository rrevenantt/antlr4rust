//use tree::RuleNode;

use std::any::Any;
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use crate::atn::INVALID_ALT;
use crate::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, ParserRuleContextType};

//pub trait RuleContext:RuleNode {
pub trait RuleContext {
    fn get_invoking_state(&self) -> isize;
    fn set_invoking_state(&self, t: isize);

    fn is_empty(&self) -> bool {
        self.get_invoking_state() == -1
    }

    fn get_parent_ctx(&self) -> Option<Rc<dyn ParserRuleContext>>;

    fn set_parent(&self, parent: &Option<Rc<dyn ParserRuleContext>>);
}

pub struct EmptyCustomRuleContext;

impl CustomRuleContext for EmptyCustomRuleContext {
    fn get_rule_index(&self) -> usize {
        usize::max_value()
    }
}

pub trait CustomRuleContext: 'static {
    fn get_rule_index(&self) -> usize;
    fn get_alt_number(&self) -> isize { INVALID_ALT }
    fn set_alt_number(&self, _alt_number: isize) {}
    fn enter(_ctx: &BaseParserRuleContext<Self>, _listener: &mut dyn Any) where Self: Sized {}
    fn exit(_ctx: &BaseParserRuleContext<Self>, _listener: &mut dyn Any) where Self: Sized {}
}

pub struct BaseRuleContext<Ctx: CustomRuleContext> {
    pub(crate) parent_ctx: RefCell<Option<Weak<dyn ParserRuleContext>>>,
    invoking_state: Cell<isize>,
    pub(crate) ext: Ctx,
}

impl<Ctx: CustomRuleContext> BaseRuleContext<Ctx> {
    pub(crate) fn new_ctx(parent_ctx: Option<ParserRuleContextType>, invoking_state: isize, ext: Ctx) -> Self {
        BaseRuleContext {
            parent_ctx: RefCell::new(parent_ctx.as_ref().map(Rc::downgrade)),
            invoking_state: Cell::new(invoking_state),
            ext,
        }
    }
}

impl<Ctx: CustomRuleContext> RuleContext for BaseRuleContext<Ctx> {
    fn get_invoking_state(&self) -> isize {
        self.invoking_state.get()
    }

    fn set_invoking_state(&self, t: isize) {
        self.invoking_state.set(t)
    }

    fn get_parent_ctx(&self) -> Option<Rc<dyn ParserRuleContext>> {
        self.parent_ctx.borrow().as_ref().map(Weak::upgrade).flatten()
    }

//    fn get_parent_ctx(&self) -> Option<ParserRuleContextType> {
//        self.parent_ctx.borrow().as_ref().map(Weak::upgrade).map(Option::unwrap)
//    }

    fn set_parent(&self, parent: &Option<Rc<dyn ParserRuleContext>>) {
        *self.parent_ctx.borrow_mut() = parent.as_ref().map(Rc::downgrade);
    }
}
