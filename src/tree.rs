use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Error, Formatter};
use std::ops::Deref;

use crate::atn::INVALID_ALT;
use crate::int_stream::EOF;
use crate::interval_set::Interval;
use crate::parser::Parser;
use crate::parser_rule_context::{BaseParserRuleContext, cast, ParserRuleContext, ParserRuleContextType};
use crate::rule_context::CustomRuleContext;
use crate::token::{OwningToken, Token};
use crate::trees;

//todo try to make in more generic
pub trait Tree: NodeText {
    fn get_parent(&self) -> Option<ParserRuleContextType>;
    fn has_parent(&self) -> bool;
    fn get_payload(&self) -> Box<dyn Any>;
    fn get_child(&self, i: usize) -> Option<ParserRuleContextType>;
    fn get_child_count(&self) -> usize;
    fn get_children(&self) -> Ref<Vec<ParserRuleContextType>>;
    fn get_children_full(&self) -> &RefCell<Vec<ParserRuleContextType>>;
}

pub trait ParseTree: Tree {
    /// Returns interval in input string which corresponds to this subtree
    fn get_source_interval(&self) -> Interval;

    /// Return combined text of this AST node.
    /// To create resulting string it does traverse whole subtree,
    /// also it includes only tokens added to the parse tree
    ///
    /// Since tokens on hidden channels (e.g. whitespace or comments) are not
    ///	added to the parse trees, they will not appear in the output of this
    ///	method.
    fn get_text(&self) -> String;

    /// Print out a whole tree, not just a node, in LISP format
    /// (root child1 .. childN). Print just a node if this is a leaf.
    /// We have to know the recognizer so we can get rule names.
    fn to_string_tree(&self, r: &dyn Parser) -> String {
        trees::string_tree(self, r.get_rule_names())
    }
}

pub trait NodeText {
    fn get_node_text(&self, rule_names: &[&str]) -> String;
}

impl<T: Tree> NodeText for T {
    default fn get_node_text(&self, _rule_names: &[&str]) -> String {
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

//todo unify code for terminal and error nodes

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
    fn get_node_text(&self, _rule_names: &[&str]) -> String {
        self.symbol.get_text().to_owned()
    }
}

impl ParseTree for TerminalNode {
    fn get_text(&self) -> String {
        self.symbol.text.to_owned()
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

impl NodeText for ErrorNode {
    fn get_node_text(&self, _rule_names: &[&str]) -> String {
        self.symbol.get_text().to_owned()
    }
}

impl ParseTree for ErrorNode {
    fn get_text(&self) -> String {
        self.symbol.text.to_owned()
    }
}


impl Debug for BaseParserRuleContext<TerminalNodeCtx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(if self.symbol.get_token_type() == EOF {
            "<EOF>"
        } else {
            self.symbol.get_text()
        }
        );
        Ok(())
    }
}

impl Debug for BaseParserRuleContext<ErrorNodeCtx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(if self.symbol.get_token_type() == EOF {
            "<EOF>"
        } else {
            self.symbol.get_text()
        }
        );
        Ok(())
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
    fn visit_terminal(&mut self, _node: &TerminalNode) {}
    fn visit_error_node(&mut self, _node: &ErrorNode) {}
    fn enter_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
    fn exit_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
}

//impl<T:ParseTreeListener> AsRef<dyn ParseTreeListener> for T{
//    fn as_ref(&self) -> &dyn ParseTreeListener {
//        self
//    }
//}

/// Helper struct to accept parse listener on already generated tree
pub struct ParseTreeWalker;

impl ParseTreeWalker {
//    fn new() -> ParseTreeWalker { ParseTreeWalker }

    pub fn walk<T: ParseTreeListener + ?Sized, Ctx: ParserRuleContext + ?Sized>(&self, listener: &mut Box<T>, t: &Ctx) {
        if t.type_id() == TypeId::of::<ErrorNode>() {
            let err = cast::<_, ErrorNode>(t);
            listener.visit_error_node(err);
            return
        }
        if t.type_id() == TypeId::of::<TerminalNode>() {
            let leaf = cast::<_, TerminalNode>(t);
            listener.visit_terminal(leaf);
            return
        }

        listener.enter_every_rule(t.upcast());
        t.enter_rule(listener as &mut dyn Any);

        for child in t.get_children().iter() {
            self.walk(listener, child.deref())
        }

        t.exit_rule(listener as &mut dyn Any);
        listener.exit_every_rule(t.upcast());
    }

}
