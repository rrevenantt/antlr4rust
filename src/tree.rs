use std::any::Any;
use std::cell::Ref;
use std::ops::Deref;
use std::rc::Rc;

use crate::atn::INVALID_ALT;
use crate::interval_set::Interval;
use crate::parser::{Parser, ParserRecog};
use crate::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, ParserRuleContextType};
use crate::recognizer::Recognizer;
use crate::rule_context::CustomRuleContext;
use crate::token::{OwningToken, Token};

//todo try to make in more generic
pub trait Tree: NodeText {
    fn get_parent(&self) -> Option<&ParserRuleContextType>;
    //    fn set_parent(&self, tree: Self);
    fn get_payload(&self) -> Box<dyn Any>;
    fn get_child(&self, i: usize) -> Option<ParserRuleContextType>;
    fn get_child_count(&self) -> usize;
    fn get_children(&self) -> Ref<Vec<ParserRuleContextType>>;
}

pub trait SyntaxTree: Tree {
    fn get_source_interval(&self) -> Interval;
}

pub trait ParseTree: SyntaxTree {
    //    fn accept(&self, v: ParseTreeVisitor) -> interface;
    fn get_text(&self) -> String;

    fn to_string_tree(&self, r: &dyn Parser) -> String;
}

pub trait NodeText {
    fn get_node_text(&self, rule_names: &[&str]) -> String;
}

impl<T: Tree> NodeText for T {
    default fn get_node_text(&self, rule_names: &[&str]) -> String {
        "<unknown>".to_owned()
    }
}

impl<T: ParserRuleContext> NodeText for T {
    default fn get_node_text(&self, rule_names: &[&str]) -> String {
        let rule_index = self.get_rule_index();
        let rule_name = rule_names[rule_index];
        let alt_number = self.get_alt_number();
        if alt_number != INVALID_ALT {
            return format!("{}:{}", rule_name, alt_number);
        }
        return rule_name.to_owned();
    }
}

//pub trait RuleNode: ParseTree {
//    fn get_rule_context(&self) -> RuleContext;
//    fn get_base_rule_context(&self) -> * BaseRuleContext;
//}
/// AST leaf
pub type TerminalNode = BaseParserRuleContext<TerminalNodeCtx>;

pub struct TerminalNodeCtx {
    pub symbol: OwningToken
}

impl CustomRuleContext for TerminalNodeCtx {
    fn get_rule_index(&self) -> usize {
        unimplemented!()
    }
}

impl NodeText for TerminalNode {
    fn get_node_text(&self, rule_names: &[&str]) -> String {
        self.symbol.get_text().to_owned()
    }
}


/// # Error Leaf
/// Created for each token created or consumed during recovery
pub type ErrorNode = BaseParserRuleContext<ErrorNodeCtx>;

//not type alias because we would like to use it in downcasting
pub struct ErrorNodeCtx(pub TerminalNodeCtx);

impl CustomRuleContext for ErrorNodeCtx {
    fn get_rule_index(&self) -> usize {
        unimplemented!()
    }
}

impl Deref for ErrorNodeCtx {
    type Target = TerminalNodeCtx;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//pub trait TerminalNode: ParseTree {
//    fn get_symbol(&self) -> &dyn Token;
//}

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
    fn visit_terminal(&self, node: &TerminalNode) {}
    fn visit_error_node(&self, node: &ErrorNode) {}
    fn enter_every_rule(&self, ctx: &ParserRuleContext) {}
    fn exit_every_rule(&self, ctx: &ParserRuleContext) {}
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
//    fn to_string_tree(&self, s Vec<String>, r: Recognizer) -> String { unimplemented!() }
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