//use tree::RuleNode;

use crate::atn::INVALID_ALT;
use crate::parser_rule_context::ParserRuleContext;
use crate::tree::ParseTreeListener;

//pub trait RuleContext:RuleNode {
pub trait RuleContext {
    fn get_invoking_state(&self) -> isize;
    fn set_invoking_state(&self, t: isize);

    fn is_empty(&self) -> bool;


    //todo rewrite into take and get
    fn get_parent_ctx(&mut self) -> &mut Option<Box<dyn ParserRuleContext>>;
    fn peek_parent(&self) -> Option<&dyn ParserRuleContext>;
}

pub(crate) struct CustomRuleContextInternal;

impl CustomRuleContext for CustomRuleContextInternal {
    fn get_rule_index(&self) -> usize {
        usize::max_value()
    }
}

pub trait CustomRuleContext: Sync + Send {
    fn get_rule_index(&self) -> usize;
    fn get_alt_number(&self) -> isize { INVALID_ALT }
    fn set_alt_number(&self, alt_number: isize) {}
//    fn enter(&self, listener: &dyn ParseTreeListener) ;
//
//    fn exit(&self, listener: &dyn ParseTreeListener) ;
}


pub struct BaseRuleContext<Ctx: CustomRuleContext> {
    parent_ctx: Option<Box<dyn ParserRuleContext>>,
    invoking_state: isize,
    pub(crate)ext: Ctx
}

impl<Ctx: CustomRuleContext> BaseRuleContext<Ctx> {
    pub(crate) fn new_ctx(parent_ctx: Option<Box<dyn ParserRuleContext>>, invoking_state: isize, ext: Ctx) -> Self {
        BaseRuleContext {
            parent_ctx,
            invoking_state,
            ext
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

    fn get_parent_ctx(&mut self) -> &mut Option<Box<dyn ParserRuleContext>> {
        &mut self.parent_ctx
    }

    fn peek_parent(&self) -> Option<&ParserRuleContext> {
        self.parent_ctx.as_deref()
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
