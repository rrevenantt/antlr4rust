use crate::interval_set::Interval;
use std::any::Any;
use crate::parser_rule_context::ParserRuleContext;
use crate::token::Token;

pub trait Tree {
    fn get_parent(&self) -> &dyn Tree;
    //    fn set_parent(&self, tree: Self);
    fn get_payload(&self) -> Box<dyn Any>;
    fn get_child(&self, i: isize) -> &dyn Tree;
    fn get_child_count(&self) -> isize;
    fn get_children(&self) -> Vec<&dyn Tree>;
}

pub trait SyntaxTree: Tree {
    fn get_source_interval(&self) -> Interval;
}

pub trait ParseTree: SyntaxTree {
    //    fn accept(&self, v: ParseTreeVisitor) -> interface;
    fn get_text(&self) -> String;

//    fn to_String_tree(&self, s: Vec<String>, r: Recognizer) -> String;
}

//pub trait RuleNode: ParseTree {
//    fn get_rule_context(&self) -> RuleContext;
//    fn get_base_rule_context(&self) -> * BaseRuleContext;
//}

pub trait TerminalNode: ParseTree {
    fn get_symbol(&self) -> &dyn Token;
}

//pub trait ErrorNode: TerminalNode {
//    fn error_node(&self);
//}

//pub trait ParseTreeVisitor {
//    fn visit(&self, tree: &ParseTree) -> interface;
//    fn visit_children(&self, node: &RuleNode) -> interface;
//    fn visit_terminal(&self, node: &TerminalNode) -> interface;
//    fn visit_error_node(&self, node: &ErrorNode) -> interface;
//}
//
//pub struct BaseParseTreeVisitor {  }
//
//impl BaseParseTreeVisitor {
//    fn visit(&self, tree: ParseTree) -> interface { unimplemented!() }
//    fn visit_children(&self, node: RuleNode) -> interface { unimplemented!() }
//    fn visit_terminal(&self, node: TerminalNode) -> interface { unimplemented!() }
//    fn visit_error_node(&self, node: ErrorNode) -> interface { unimplemented!() }
//}

pub trait ParseTreeListener: 'static {
    //    fn visit_terminal(&self, node: TerminalNode) {}
//    fn visit_error_node(&self, node: ErrorNode) {}
    fn enter_every_rule(&self, ctx: &ParserRuleContext) {}
    fn exit_every_rule(&self, ctx: &ParserRuleContext) {}
}

pub trait Guard {
    type T: ParseTreeListener + ?Sized;
}

pub trait ListenerDispatch {
    fn enter(&mut self, ctx: &ParserRuleContext);
    fn exit(&mut self, ctx: &ParserRuleContext);
}

//pub struct BaseParseTreeListener {  }
//
//impl ParseTreeListener for BaseParseTreeListener {
//    fn visit_terminal(&self, node: TerminalNode) { unimplemented!() }
//
//    fn visit_error_node(&self, node: ErrorNode) { unimplemented!() }
//
//    fn enter_every_rule(&self, ctx: ParserRuleContext) { unimplemented!() }
//
//    fn exit_every_rule(&self, ctx: ParserRuleContext) { unimplemented!() }
//}
//pub struct TerminalNodeImpl {
//    parent_ctx: RuleContext,
//    symbol: Token,
//}
//
//impl TerminalNodeImpl {
//    fn new_terminal_node_impl(symbol Token) -> * TerminalNodeImpl { unimplemented!() }
//
//    fn get_child(&self, i: isize) -> Tree { unimplemented!() }
//
//    fn get_children(&self) -> Vec<Tree> { unimplemented!() }
//
//    fn set_children(&self, tree Vec<Tree>) { unimplemented!() }
//
//    fn get_symbol(&self) -> Token { unimplemented!() }
//
//    fn get_parent(&self) -> Tree { unimplemented!() }
//
//    fn set_parent(&self, tree: Tree) { unimplemented!() }
//
//    fn get_payload(&self) -> interface { unimplemented!() } {
//    return t.symbol
//    }
//
//    fn get_source_interval(&self) -> * Interval { unimplemented!() }
//
//    fn get_child_count(&self) -> int { unimplemented!() }
//
//    fn accept(&self, v: ParseTreeVisitor) -> interface { unimplemented!() } {
//    return v.VisitTerminal(t)
//    }
//
//    fn get_text(&self) -> String { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//    fn to_String_tree(&self, s Vec<String>, r: Recognizer) -> String { unimplemented!() }
//}
//
//pub struct ErrorNodeImpl {
//    base: TerminalNodeImpl,
//}
//
//impl ErrorNodeImpl {
//    fn new_error_node_impl(token Token) -> * ErrorNodeImpl { unimplemented!() }
//
//    fn error_node(&self) { unimplemented!() }
//
//    fn accept(&self, v: ParseTreeVisitor) -> interface { unimplemented!() } {
//    return v.VisitErrorNode(e)
//    }
//}
//pub struct ParseTreeWalker { }
//
//impl ParseTreeWalker {
//    fn new_parse_tree_walker() -> * ParseTreeWalker { unimplemented!() }
//
//    fn walk(&self, listener: ParseTreeListener, t: Tree) { unimplemented!() }
//
//    fn enter_rule(&self, listener: ParseTreeListener, r: RuleNode) { unimplemented!() }
//
//    fn exit_rule(&self, listener: ParseTreeListener, r: RuleNode) { unimplemented!() }
//
////var ParseTreeWalkerDefault = NewParseTreeWalker()
//}
//