//use tree::RuleNode;

use std::borrow::{BorrowMut, Cow};
use std::cell::{Cell, Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::atn::INVALID_ALT;
use crate::parser_rule_context::{ParserRuleContext, ParserRuleContextType};
use crate::tree::ParseTreeListener;

//pub trait RuleContext:RuleNode {
pub trait RuleContext {
    fn get_invoking_state(&self) -> isize;
    fn set_invoking_state(&self, t: isize);

    fn is_empty(&self) -> bool;


    //todo rewrite into take and get
    fn get_parent_ctx(&self) -> Option<Rc<dyn ParserRuleContext>>;
    fn peek_parent(&self) -> Option<ParserRuleContextType>;
    fn set_parent(&self, parent: &Option<Rc<dyn ParserRuleContext>>);
}

pub(crate) struct EmptyCustomRuleContext;

impl CustomRuleContext for EmptyCustomRuleContext {
    fn get_rule_index(&self) -> usize {
        usize::max_value()
    }
}

pub trait CustomRuleContext {
    fn get_rule_index(&self) -> usize;
    fn get_alt_number(&self) -> isize { INVALID_ALT }
    fn set_alt_number(&self, alt_number: isize) {}
//    fn enter(&self, listener: &dyn ParseTreeListener) ;
//
//    fn exit(&self, listener: &dyn ParseTreeListener) ;
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

    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn get_parent_ctx(&self) -> Option<Rc<dyn ParserRuleContext>> {
        self.parent_ctx.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn peek_parent(&self) -> Option<ParserRuleContextType> {
        self.parent_ctx.borrow().as_ref().map(Weak::upgrade).map(Option::unwrap)
    }

    fn set_parent(&self, parent: &Option<Rc<dyn ParserRuleContext>>) {
        *self.parent_ctx.borrow_mut() = parent.as_ref().map(Rc::downgrade);
    }
}

//pub struct TreeOwner<T>{
//    nodes:Vec<T>,
//    first:usize
//}
//
//struct NodeData<T>{
//    data:T,
//    parent:NodeRef,
//    children:Vec<NodeRef>
//}
//
//pub struct NodeRef{
//    ptr: usize,
//}
