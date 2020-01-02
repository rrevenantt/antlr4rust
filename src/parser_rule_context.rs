use std::any::Any;
use std::cell::{Cell, Ref, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::sync::Arc;

use crate::errors::ANTLRError;
use crate::interval_set::Interval;
use crate::parser::{Parser, ParserRecog};
use crate::recognizer::Recognizer;
use crate::rule_context::{BaseRuleContext, CustomRuleContext, EmptyCustomRuleContext, RuleContext};
use crate::token::{OwningToken, Token};
use crate::tree::{ErrorNode, ParseTree, ParseTreeListener, SyntaxTree, TerminalNode, Tree};
use crate::trees;

pub trait ParserRuleContext: RuleContext + CustomRuleContext + ParseTree {
    fn set_exception(&mut self, e: ANTLRError);

    fn set_start(&self, t: isize);
    fn get_start(&self) -> isize;

    fn set_stop(&self, t: isize);
    fn get_stop(&self) -> isize;


    fn add_token_node(&self, token: TerminalNode) -> Rc<dyn ParserRuleContext>;
    fn add_error_node(&self, bad_token: ErrorNode) -> Rc<dyn ParserRuleContext>;

    fn add_child(&self, child: ParserRuleContextType);
    fn remove_last_child(&self);
}

//requires ParserRuleContext to be Sync
//lazy_static! {
//    pub static ref EMPTY_CTX: Box<dyn ParserRuleContext> =
//        Box::new(BaseParserRuleContext::new_parser_ctx(None,-1,CustomRuleContextInternal));
//}

//todo do not calc this every time, maybe threadlocal?
pub(crate) fn empty_ctx() -> Box<dyn ParserRuleContext> {
    Box::new(BaseParserRuleContext::new_parser_ctx(None, -1, EmptyCustomRuleContext))
}

pub type ParserRuleContextType = Rc<dyn ParserRuleContext>;

pub struct BaseParserRuleContext<Ctx: CustomRuleContext> {
    base: BaseRuleContext<Ctx>,

    start: Cell<isize>,
    stop: Cell<isize>,
    exception: Option<Box<ANTLRError>>,
    /// List of children of current node
    /// Editing is done via Rc::try_unwrap or Rc::get_mut
    /// because in well-formed tree strong ref count should be one
    /// But Rc is still needed for Weak references to parent
    pub(crate) children: RefCell<Vec<ParserRuleContextType>>,
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

    fn get_parent_ctx(&self) -> Option<Rc<dyn ParserRuleContext>> {
        self.base.get_parent_ctx()
    }

    fn peek_parent(&self) -> Option<ParserRuleContextType> {
        self.base.peek_parent()
    }

    fn set_parent(&self, parent: &Option<Rc<dyn ParserRuleContext>>) {
        self.base.set_parent(parent)
    }
}

impl<Ctx: CustomRuleContext> CustomRuleContext for BaseParserRuleContext<Ctx> {
    fn get_rule_index(&self) -> usize {
        self.base.ext.get_rule_index()
    }
}

impl<Ctx: CustomRuleContext> Deref for BaseParserRuleContext<Ctx> {
    type Target = Ctx;

    fn deref(&self) -> &Self::Target {
        &self.base.ext
    }
}

impl<Ctx: CustomRuleContext> ParserRuleContext for BaseParserRuleContext<Ctx> {
    fn set_exception(&mut self, e: ANTLRError) {
        self.exception = Some(Box::new(e));
    }

    fn set_start(&self, t: isize) {
        self.start.set(t);
    }

    fn get_start(&self) -> isize {
        self.start.get()
    }

    fn set_stop(&self, t: isize) {
        self.stop.set(t);
    }

    fn get_stop(&self) -> isize {
        self.stop.get()
    }

    fn add_token_node(&self, token: TerminalNode) -> Rc<dyn ParserRuleContext> {
        let node: Rc<dyn ParserRuleContext> = Rc::new(token);
        self.children.borrow_mut().push(node.clone());
        node
    }

    fn add_error_node(&self, bad_token: ErrorNode) -> Rc<dyn ParserRuleContext> {
//        bad_token.base.parent_ctx =
        let node: Rc<dyn ParserRuleContext> = Rc::new(bad_token);

        self.children.borrow_mut().push(node.clone());
        node
    }

    fn add_child(&self, child: ParserRuleContextType) {
        self.children.borrow_mut().push(child);
    }

    fn remove_last_child(&self) {
        self.children.borrow_mut().pop();
    }
}

impl<Ctx: CustomRuleContext> BaseParserRuleContext<Ctx> {
    pub fn new_parser_ctx(parent_ctx: Option<ParserRuleContextType>, invoking_state: isize, ext: Ctx) -> Self {
        BaseParserRuleContext {
            base: BaseRuleContext::new_ctx(parent_ctx, invoking_state, ext),
            start: Cell::new(-1),
            stop: Cell::new(-1),
            exception: None,
            children: RefCell::new(vec![]),
        }
    }
}

impl<Ctx: CustomRuleContext> Tree for BaseParserRuleContext<Ctx> {
    fn get_parent(&self) -> Option<&ParserRuleContextType> {
        unimplemented!()
    }

    fn get_payload(&self) -> Box<dyn Any> {
        unimplemented!()
    }

    fn get_child(&self, i: usize) -> Option<ParserRuleContextType> {
        self.children.borrow().get(i).cloned()
    }

    fn get_child_count(&self) -> usize {
        self.children.borrow().len()
    }

    fn get_children(&self) -> Ref<Vec<ParserRuleContextType>> {
        self.children.borrow()
    }
}

impl<Ctx: CustomRuleContext> SyntaxTree for BaseParserRuleContext<Ctx> {
    fn get_source_interval(&self) -> Interval {
        unimplemented!()
    }
}

impl<Ctx: CustomRuleContext> ParseTree for BaseParserRuleContext<Ctx> {
    fn get_text(&self) -> String {
        unimplemented!()
    }

    fn to_string_tree(&self, r: &dyn Parser) -> String {
        trees::string_tree(self, r.get_rule_names())
    }
}
//    fn copy_from(&self, ctx: * BaseParserRuleContext) { unimplemented!() }
//
//    fn get_text(&self) -> String { unimplemented!() }
//
//    fn add_terminal_node_child(&self, child: TerminalNode) -> TerminalNode { unimplemented!() }
//
//    fn get_child_of_type(&self, i: isize, childType: reflect.Type) -> RuleContext { unimplemented!() }
//
//    fn to_string_tree(&self, ruleNames Vec<String>, recog: Recognizer) -> String { unimplemented!() }
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
 