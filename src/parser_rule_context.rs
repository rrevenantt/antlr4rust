use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::errors::ANTLRError;
use crate::rule_context::{BaseRuleContext, CustomRuleContext, CustomRuleContextInternal, RuleContext};
use crate::token::{OwningToken, Token};
use crate::tree::ParseTreeListener;

pub trait ParserRuleContext: CustomRuleContext + RuleContext /*+ Send + Sync*/ {
    fn set_exception(&mut self, e: ANTLRError);

    fn set_start(&mut self, t: isize);
    fn get_start(&self) -> isize;

    fn set_stop(&mut self, t: isize);
    fn get_stop(&self) -> isize;


//    fn add_token_node(&self, token: Box<dyn Token>) -> * TerminalNodeImpl;
//    fn add_error_node(&self, badToken: Box<dyn Token>) -> * ErrorNodeImpl;

//    fn add_child(&self, child: Self) -> RuleContext;
//    fn remove_last_child(&self);
}

//requires ParserRuleContext to be Sync
//lazy_static! {
//    pub static ref EMPTY_CTX: Box<dyn ParserRuleContext> =
//        Box::new(BaseParserRuleContext::new_parser_ctx(None,-1,CustomRuleContextInternal));
//}
//todo do not calc this every time, maybe threadlocal?
pub(crate) fn empty_ctx() -> Box<dyn ParserRuleContext> {
    Box::new(BaseParserRuleContext::new_parser_ctx(None, -1, CustomRuleContextInternal))
}


pub struct BaseParserRuleContext<Ctx: CustomRuleContext> {
    base: BaseRuleContext<Ctx>,

    start: isize,
    stop: isize,
    exception: Option<Box<ANTLRError>>,
//    children: Vec<Tree>,
}

impl<Ctx: CustomRuleContext> RuleContext for BaseParserRuleContext<Ctx> {
    fn get_invoking_state(&self) -> isize {
        self.base.get_invoking_state()
    }

    fn set_invoking_state(&self, t: isize) {
        self.base.set_invoking_state(t)
    }

    fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    fn get_parent_ctx(&mut self) -> &mut Option<Box<dyn ParserRuleContext>> {
        self.base.get_parent_ctx()
    }

    fn peek_parent(&self) -> Option<&dyn ParserRuleContext> {
        self.base.peek_parent()
    }
}

impl<Ctx: CustomRuleContext> CustomRuleContext for BaseParserRuleContext<Ctx> {
    fn get_rule_index(&self) -> usize {
        self.base.ext.get_rule_index()
    }
}

impl<Ctx: CustomRuleContext> ParserRuleContext for BaseParserRuleContext<Ctx> {
    fn set_exception(&mut self, e: ANTLRError) {
        self.exception = Some(Box::new(e));
    }

    fn set_start(&mut self, t: isize) {
        self.start = t;
    }

    fn get_start(&self) -> isize {
        self.start
    }

    fn set_stop(&mut self, t: isize) {
        self.stop = t;
    }

    fn get_stop(&self) -> isize {
        self.stop
    }
}

impl<Ctx: CustomRuleContext> BaseParserRuleContext<Ctx> {
    pub fn new_parser_ctx(parent_ctx: Option<Box<dyn ParserRuleContext>>, invoking_state: isize, ext: Ctx) -> Self {
        BaseParserRuleContext {
            base: BaseRuleContext::new_ctx(parent_ctx, invoking_state, ext),
            start: -1,
            stop: -1,
            exception: None,
        }
    }


//
//    fn set_exception(&self, e: RecognitionError) { unimplemented!() }
//
//    fn get_children(&self) -> Vec<Tree> { unimplemented!() }
//
//    fn copy_from(&self, ctx: * BaseParserRuleContext) { unimplemented!() }
//
//    fn get_text(&self) -> String { unimplemented!() }
//
//    fn enter_rule(&self, listener: ParseTreeListener) {  }
//
//    fn exit_rule(&self, listener: ParseTreeListener) { unimplemented!() }
//
//    fn add_terminal_node_child(&self, child: TerminalNode) -> TerminalNode { unimplemented!() }
//
//    fn add_child(&self, child: RuleContext) -> RuleContext { unimplemented!() }
//
//    fn remove_last_child(&self) { unimplemented!() }
//
//    fn add_token_node(&self, token: Token) -> * TerminalNodeImpl { unimplemented!() }
//
//    fn add_error_node(&self, badToken: Token) -> * ErrorNodeImpl { unimplemented!() }
//
//    fn get_child(&self, i: isize) -> Tree { unimplemented!() }
//
//    fn get_child_of_type(&self, i: isize, childType: reflect.Type) -> RuleContext { unimplemented!() }
//
//    fn to_String_tree(&self, ruleNames Vec<String>, recog: Recognizer) -> String { unimplemented!() }
//
//    fn get_rule_context(&self) -> RuleContext { unimplemented!() }
//
//    fn accept(&self, visitor: ParseTreeVisitor) -> interface { unimplemented!() } {
//    return visitor.VisitChildren(prc)
//    }
//
//    fn get_token(&self, ttype: isize, i: isize) -> TerminalNode { unimplemented!() }
//
//    fn get_tokens(&self, ttype: isize) -> Vec<TerminalNode> { unimplemented!() }
//
//    fn get_payload(&self) -> interface { unimplemented!() } {
//    return: prc,
//    }
//
//    fn get_child(&self, ctxType: reflect.Type, i: isize) -> RuleContext { unimplemented!() }
//
//
//    fn get_typed_rule_context(&self, ctxType: reflect.Type, i: isize) -> RuleContext { unimplemented!() }
//
//    fn get_typed_rule_contexts(&self, ctxType: reflect.Type) -> Vec<RuleContext> { unimplemented!() }
//
//    fn get_child_count(&self) -> int { unimplemented!() }
//
//    fn get_source_interval(&self) -> * Interval { unimplemented!() }
//
//
//    fn String(&self, ruleNames Vec<String>, stop: RuleContext) -> String { unimplemented!() }
//
//    var RuleContextEmpty = NewBaseParserRuleContext(nil, - 1)
//
//    pub trait InterpreterRuleContext {
//    parser_rule_context
//    }
//
//    pub struct BaseInterpreterRuleContext {
//    base: BaseParserRuleContext,
//    }
//
//    fn new_base_interpreter_rule_context(parent BaseInterpreterRuleContext, invokingStateNumber: isize, ruleIndex: isize) -> * BaseInterpreterRuleContext { unimplemented!() }
}
 