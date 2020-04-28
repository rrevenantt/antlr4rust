use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Error, Formatter};
use std::marker::PhantomData;
use std::ops::{CoerceUnsized, Deref};
use std::rc::Rc;

use crate::atn::INVALID_ALT;
use crate::int_stream::EOF;
use crate::interval_set::Interval;
use crate::parser::Parser;
use crate::parser_rule_context::{BaseParserRuleContext, cast, ParserRuleContext, ParserRuleContextType};
use crate::recognizer::Recognizer;
use crate::rule_context::{CustomRuleContext, Tid};
use crate::token::{OwningToken, Token};
use crate::token_factory::{CommonTokenFactory, TokenFactory};
use crate::trees;

//todo try to make in more generic
pub trait Tree<'input>: NodeText + CustomRuleContext<'input> {
    fn get_parent(&self) -> Option<ParserRuleContextType<'input, Self::TF>>;
    fn has_parent(&self) -> bool;
    fn get_payload(&self) -> Box<dyn Any>;
    fn get_child(&self, i: usize) -> Option<ParserRuleContextType<'input, Self::TF>>;
    fn get_child_count(&self) -> usize;
    fn get_children(&self) -> Ref<Vec<ParserRuleContextType<'input, Self::TF>>>;
    fn get_children_full(&self) -> &RefCell<Vec<ParserRuleContextType<'input, Self::TF>>>;
}

pub trait ParseTree<'input>: Tree<'input> {
    /// Return an {@link Interval} indicating the index in the
    /// {@link TokenStream} of the first and last token associated with this
    /// subtree. If this node is a leaf, then the interval represents a single
    /// token and has interval i..i for token index i.
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
    fn to_string_tree(&self, r: &dyn Recognizer<'input, TF=Self::TF>) -> String {
        trees::string_tree(self, r.get_rule_names())
    }
}

pub trait NodeText {
    fn get_node_text(&self, rule_names: &[&str]) -> String;
}

impl<T> NodeText for T {
    default fn get_node_text(&self, _rule_names: &[&str]) -> String {
        "<unknown>".to_owned()
    }
}

impl<'input, T: CustomRuleContext<'input>> NodeText for T {
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
pub type TerminalNode<'input, TF> = BaseParserRuleContext<'input, TerminalNodeCtx<'input, TF>>;

pub struct TerminalNodeCtx<'input, TF: TokenFactory<'input> + 'input> {
    pub symbol: TF::Tok
}

impl<'input, TF: TokenFactory<'input> + 'input> CustomRuleContext<'input> for TerminalNodeCtx<'input, TF> {
    type TF = TF;

    fn get_rule_index(&self) -> usize {
        usize::max_value()
    }
}

unsafe impl<'input, TF: TokenFactory<'input> + 'input> Tid for TerminalNodeCtx<'input, TF> {
    fn self_id(&self) -> TypeId {
        TypeId::of::<TerminalNodeCtx<'static, CommonTokenFactory>>()
    }

    fn id() -> TypeId where Self: Sized {
        TypeId::of::<TerminalNodeCtx<'static, CommonTokenFactory>>()
    }
}

impl<'input, TF: TokenFactory<'input> + 'input> NodeText for TerminalNode<'input, TF> {
    fn get_node_text(&self, _rule_names: &[&str]) -> String {
        self.symbol.borrow().get_text().to_owned()
    }
}

impl<'input, TF: TokenFactory<'input> + 'input> ParseTree<'input> for TerminalNode<'input, TF> {
    fn get_text(&self) -> String {
        self.symbol.borrow().get_text().to_owned()
    }
}


/// # Error Leaf
/// Created for each token created or consumed during recovery
pub type ErrorNode<'input, TF> = BaseParserRuleContext<'input, ErrorNodeCtx<'input, TF>>;

//not type alias because we would like to use it in downcasting
pub struct ErrorNodeCtx<'input, TF: TokenFactory<'input> + 'input>(pub TerminalNodeCtx<'input, TF>);

impl<'input, TF: TokenFactory<'input> + 'input> CustomRuleContext<'input> for ErrorNodeCtx<'input, TF> {
    type TF = TF;

    fn get_rule_index(&self) -> usize {
        usize::max_value() - 1
    }
}

unsafe impl<'input, TF: TokenFactory<'input> + 'input> Tid for ErrorNodeCtx<'input, TF> {
    fn self_id(&self) -> TypeId {
        TypeId::of::<ErrorNodeCtx<'static, CommonTokenFactory>>()
    }

    fn id() -> TypeId where Self: Sized {
        TypeId::of::<ErrorNodeCtx<'static, CommonTokenFactory>>()
    }
}

impl<'input, TF: TokenFactory<'input> + 'input> Deref for ErrorNodeCtx<'input, TF> {
    type Target = TerminalNodeCtx<'input, TF>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'input, TF: TokenFactory<'input> + 'input> NodeText for ErrorNode<'input, TF> {
    fn get_node_text(&self, _rule_names: &[&str]) -> String {
        self.symbol.borrow().get_text().to_owned()
    }
}

impl<'input, TF: TokenFactory<'input> + 'input> ParseTree<'input> for ErrorNode<'input, TF> {
    fn get_text(&self) -> String {
        self.symbol.borrow().get_text().to_owned()
    }
}


impl<'input, TF: TokenFactory<'input> + 'input> Debug for BaseParserRuleContext<'input, TerminalNodeCtx<'input, TF>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(
            if self.symbol.borrow().get_token_type() == EOF {
                "<EOF>"
            } else {
                self.symbol.borrow().get_text()
            }
        );
        Ok(())
    }
}

impl<'input, TF: TokenFactory<'input> + 'input> Debug for BaseParserRuleContext<'input, ErrorNodeCtx<'input, TF>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(
            if self.symbol.borrow().get_token_type() == EOF {
                "<EOF>"
            } else {
                self.symbol.borrow().get_text()
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

pub trait ParseTreeListener<'input, TF: TokenFactory<'input> + 'input = CommonTokenFactory>: 'static {
    fn visit_terminal(&mut self, _node: &TerminalNode<'input, TF>) {}
    fn visit_error_node(&mut self, _node: &ErrorNode<'input, TF>) {}
    fn enter_every_rule(&mut self, _ctx: &dyn ParserRuleContext<'input, TF=TF>) {}
    fn exit_every_rule(&mut self, _ctx: &dyn ParserRuleContext<'input, TF=TF>) {}
}

//impl<T:ParseTreeListener> AsRef<dyn ParseTreeListener> for T{
//    fn as_ref(&self) -> &dyn ParseTreeListener {
//        self
//    }
//}

/// Helper struct to accept parse listener on already generated tree
pub struct ParseTreeWalker<'a, TF: TokenFactory<'a> + 'a, T: ParseTreeListener<'a, TF> + ?Sized = dyn for<'x> ParseTreeListener<'a, TF>>(PhantomData<fn(T) -> &'a TF::Tok>);

impl<'a, T: ParseTreeListener<'a, TF> + ?Sized, TF: TokenFactory<'a> + 'a> ParseTreeWalker<'a, TF, T> {
    // #[doc(hidden)]
    // pub fn new() -> Self{ Self(PhantomData) }

    pub fn walk<Ctx: ParserRuleContext<'a, TF=TF> + 'a + ?Sized, Listener>(listener: Box<Listener>, t: &Ctx) -> Box<Listener>
        where
            Box<Listener>: CoerceUnsized<Box<T>>
    {
        let mut listener = listener as Box<T>;
        Self::walk_inner(&mut listener, t);

        // just cast back
        unsafe { Box::<Listener>::from_raw(Box::into_raw(listener) as *mut _) }
    }

    fn walk_inner<Ctx: ParserRuleContext<'a, TF=TF> + 'a + ?Sized>(listener: &mut Box<T>, t: &Ctx) {
        if t.self_id() == ErrorNode::<'a, Ctx::TF>::id() {
            let err = cast::<_, ErrorNode::<'a, Ctx::TF>>(t);
            listener.visit_error_node(err);
            return;
        }
        if t.self_id() == TerminalNode::<'a, Ctx::TF>::id() {
            let leaf = cast::<_, TerminalNode::<'a, Ctx::TF>>(t);
            listener.visit_terminal(leaf);
            return;
        }

        listener.enter_every_rule(t.upcast());
        t.enter_rule(listener as &mut dyn Any);

        for child in t.get_children().iter() {
            Self::walk_inner(listener, child.deref())
        }

        t.exit_rule(listener as &mut dyn Any);
        listener.exit_every_rule(t.upcast());
    }
}
