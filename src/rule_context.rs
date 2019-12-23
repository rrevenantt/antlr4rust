//use tree::RuleNode;

use std::borrow::Cow;
use std::rc::{Rc, Weak};

use crate::atn::INVALID_ALT;
use crate::parser_rule_context::ParserRuleContext;
use crate::tree::ParseTreeListener;

//pub trait RuleContext:RuleNode {
pub trait RuleContext {
    fn get_invoking_state(&self) -> isize;
    fn set_invoking_state(&self, t: isize);

    fn is_empty(&self) -> bool;


    //todo rewrite into take and get
    fn get_parent_ctx(&self) -> &Option<Weak<dyn ParserRuleContext>>;
    fn peek_parent(&self) -> Option<Rc<dyn ParserRuleContext>>;
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
    parent_ctx: Option<Weak<dyn ParserRuleContext>>,
    invoking_state: isize,
    pub(crate)ext: Ctx,
}

impl<Ctx: CustomRuleContext> BaseRuleContext<Ctx> {
    pub(crate) fn new_ctx(parent_ctx: Option<Rc<dyn ParserRuleContext>>, invoking_state: isize, ext: Ctx) -> Self {
        BaseRuleContext {
            parent_ctx: parent_ctx.as_ref().map(Rc::downgrade),
            invoking_state,
            ext,
        }
    }
}

impl<Ctx: CustomRuleContext> RuleContext for BaseRuleContext<Ctx> {
    fn get_invoking_state(&self) -> isize {
        self.invoking_state
    }

    fn set_invoking_state(&self, t: isize) {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn get_parent_ctx(&self) -> &Option<Weak<dyn ParserRuleContext>> {
        &self.parent_ctx
    }

    fn peek_parent(&self) -> Option<Rc<dyn ParserRuleContext>> {
        self.parent_ctx.as_ref().map(Weak::upgrade).map(Option::unwrap)
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
