use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Error, Formatter, Write};
use std::ops::Deref;

use crate::atn::INVALID_ALT;
use crate::int_stream::EOF;
use crate::interval_set::Interval;
use crate::parser::Parser;
use crate::parser_rule_context::{BaseParserRuleContext, cast, ParserRuleContext, ParserRuleContextType};
use crate::rule_context::CustomRuleContext;
use crate::token::{OwningToken, Token};

//todo try to make in more generic
pub trait Tree: NodeText {
    fn get_parent(&self) -> Option<ParserRuleContextType>;
    fn has_parent(&self) -> bool;
    //    fn set_parent(&self, tree: Self);
    fn get_payload(&self) -> Box<dyn Any>;
    fn get_child(&self, i: usize) -> Option<ParserRuleContextType>;
    fn get_child_count(&self) -> usize;
    fn get_children(&self) -> Ref<Vec<ParserRuleContextType>>;
    fn get_children_full(&self) -> &RefCell<Vec<ParserRuleContextType>>;
}

pub trait ParseTree: Tree {
    //    fn accept(&self, v: ParseTreeVisitor) -> interface;
    fn get_source_interval(&self) -> Interval;

    fn get_text(&self) -> String;

    fn to_string_tree(&self, r: &dyn Parser) -> String;
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
    fn get_node_text(&self, _rule_names: &[&str]) -> String {
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

impl NodeText for ErrorNode {
    fn get_node_text(&self, _rule_names: &[&str]) -> String {
        self.symbol.get_text().to_owned()
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

//pub trait ListenerAcceptor{
//    fn enter(&self, listener: &mut dyn Any);
//    fn exit(&self, listener: &mut dyn Any);
//}
//
//impl<Ctx:CustomRuleContext> ListenerAcceptor for BaseParserRuleContext<Ctx>{
//    default fn enter(&self, listener: &mut dyn Any) {}
//    default fn exit(& self, listener: &mut dyn Any) {}
//}

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
pub struct ParseTreeWalker;

impl ParseTreeWalker {
    fn new() -> ParseTreeWalker { ParseTreeWalker }

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

//    fn enter_rule(&self, listener: ParseTreeListener, r: RuleNode) { unimplemented!() }
//
//    fn exit_rule(&self, listener: ParseTreeListener, r: RuleNode) { unimplemented!() }

//var ParseTreeWalkerDefault = NewParseTreeWalker()
}
