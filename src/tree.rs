use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Error, Formatter};
use std::iter::from_fn;
use std::marker::PhantomData;
use std::ops::{CoerceUnsized, Deref};
use std::rc::Rc;

use crate::{interval_set, trees};
use crate::atn::INVALID_ALT;
use crate::int_stream::EOF;
use crate::interval_set::Interval;
use crate::parser::{Listenable, Parser, ParserNodeType};
use crate::parser_rule_context::{BaseParserRuleContext, cast, ParserRuleContext};
use crate::recognizer::Recognizer;
use crate::rule_context::{CustomRuleContext, EmptyContextType, RuleContext, Tid};
use crate::token::{OwningToken, Token};
use crate::token_factory::{CommonTokenFactory, TokenFactory};

//todo try to make in more generic
pub trait Tree<'input>: NodeText + RuleContext<'input> {
    fn get_parent(&self) -> Option<Rc<<Self::Ctx as ParserNodeType<'input>>::Type>> { None }
    fn has_parent(&self) -> bool { false }
    fn get_payload(&self) -> Box<dyn Any> { unimplemented!() }
    fn get_child(&self, i: usize) -> Option<Rc<<Self::Ctx as ParserNodeType<'input>>::Type>> { None }
    fn get_child_count(&self) -> usize { 0 }
    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item=Rc<<Self::Ctx as ParserNodeType<'input>>::Type>> + 'a> where 'input: 'a {
        let mut index = 0;
        let iter = from_fn(move ||
            if index < self.get_child_count() {
                index += 1;
                self.get_child(index - 1)
            } else {
                None
            }
        );

        Box::new(iter)
    }
    // fn get_children_full(&self) -> &RefCell<Vec<Rc<<Self::Ctx as ParserNodeType<'input, Self::TF>>::Type>>> { unimplemented!() }
}

pub trait ParseTree<'input>: Tree<'input> {
    /// Return an {@link Interval} indicating the index in the
    /// {@link TokenStream} of the first and last token associated with this
    /// subtree. If this node is a leaf, then the interval represents a single
    /// token and has interval i..i for token index i.
    fn get_source_interval(&self) -> Interval { interval_set::INVALID }

    /// Return combined text of this AST node.
    /// To create resulting string it does traverse whole subtree,
    /// also it includes only tokens added to the parse tree
    ///
    /// Since tokens on hidden channels (e.g. whitespace or comments) are not
    ///	added to the parse trees, they will not appear in the output of this
    ///	method.
    fn get_text(&self) -> String { String::new() }

    /// Print out a whole tree, not just a node, in LISP format
    /// (root child1 .. childN). Print just a node if this is a leaf.
    /// We have to know the recognizer so we can get rule names.
    fn to_string_tree(&self, r: &dyn Recognizer<'input, TF=Self::TF, Node=Self::Ctx>) -> String {
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

#[doc(hidden)]
pub struct NoError;

#[doc(hidden)]
pub struct IsError;

pub struct LeafNode<'input, Node: ParserNodeType<'input>, T: 'static> {
    pub symbol: <Node::TF as TokenFactory<'input>>::Tok,
    iserror: PhantomData<T>,
}

impl<'input, Node: ParserNodeType<'input>, T: 'static> CustomRuleContext<'input> for LeafNode<'input, Node, T> {
    type TF = Node::TF;
    type Ctx = Node;

    fn get_rule_index(&self) -> usize {
        usize::max_value()
    }
}

impl<'input, Node: ParserNodeType<'input>, T: 'static> ParserRuleContext<'input> for LeafNode<'input, Node, T> {}

impl<'input, Node: ParserNodeType<'input>, T: 'static> Tree<'input> for LeafNode<'input, Node, T> {}

unsafe impl<'input, Node: ParserNodeType<'input>, T: 'static> Tid for LeafNode<'input, Node, T> {
    fn self_id(&self) -> TypeId {
        TypeId::of::<LeafNode<'static, EmptyContextType<CommonTokenFactory>, T>>()
    }

    fn id() -> TypeId where Self: Sized {
        TypeId::of::<LeafNode<'static, EmptyContextType<CommonTokenFactory>, T>>()
    }
}

impl<'input, Node: ParserNodeType<'input>, T: 'static> RuleContext<'input> for LeafNode<'input, Node, T> {}

impl<'input, Node: ParserNodeType<'input>, T: 'static> NodeText for LeafNode<'input, Node, T> {
    fn get_node_text(&self, _rule_names: &[&str]) -> String {
        self.symbol.borrow().get_text().to_owned()
    }
}

impl<'input, Node: ParserNodeType<'input>, T: 'static> ParseTree<'input> for LeafNode<'input, Node, T> {
    fn get_source_interval(&self) -> Interval {
        let i = self.symbol.borrow().get_token_index();
        Interval { a: i, b: i }
    }

    fn get_text(&self) -> String {
        self.symbol.borrow().get_text().to_owned()
    }
}

impl<'input, Node: ParserNodeType<'input>, T: 'static> Debug for LeafNode<'input, Node, T> {
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

impl<'input, Node: ParserNodeType<'input>, T: 'static> LeafNode<'input, Node, T> {
    pub fn new(symbol: <Node::TF as TokenFactory<'input>>::Tok) -> Self {
        Self {
            symbol,
            iserror: Default::default(),
        }
    }
}

/// AST leaf
pub type TerminalNode<'input, NodeType> = LeafNode<'input, NodeType, NoError>;

impl<'input, Node: ParserNodeType<'input>, Listener: ParseTreeListener<'input, Node> + ?Sized> Listenable<'input, Listener> for TerminalNode<'input, Node> {
    fn enter(&self, listener: &mut Listener) {
        listener.visit_terminal(self)
    }

    fn exit(&self, _listener: &mut Listener) {
        // do nothing
    }
}

/// # Error Leaf
/// Created for each token created or consumed during recovery
pub type ErrorNode<'input, NodeType> = LeafNode<'input, NodeType, IsError>;

impl<'input, Node: ParserNodeType<'input>, Listener: ParseTreeListener<'input, Node> + ?Sized> Listenable<'input, Listener> for ErrorNode<'input, Node> {
    fn enter(&self, listener: &mut Listener) {
        listener.visit_error_node(self)
    }

    fn exit(&self, _listener: &mut Listener) {
        // do nothing
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

pub trait ParseTreeListener<'input, Node: ParserNodeType<'input>> /*todo remove static*/ {
    fn visit_terminal(&mut self, _node: &TerminalNode<'input, Node>) {}
    fn visit_error_node(&mut self, _node: &ErrorNode<'input, Node>) {}
    fn enter_every_rule(&mut self, _ctx: &Node::Type) {}
    fn exit_every_rule(&mut self, _ctx: &Node::Type) {}
}

//impl<T:ParseTreeListener> AsRef<dyn ParseTreeListener> for T{
//    fn as_ref(&self) -> &dyn ParseTreeListener {
//        self
//    }
//}

/// Helper struct to accept parse listener on already generated tree
pub struct ParseTreeWalker<'input, 'a, Node, T = dyn ParseTreeListener<'input, Node> + 'a>(PhantomData<fn(&'a T) -> &'input Node::Type>)
    where Node: ParserNodeType<'input>,
          T: ParseTreeListener<'input, Node> + 'a + ?Sized;

impl<'input, 'a, Node, T> ParseTreeWalker<'input, 'a, Node, T>
    where Node: ParserNodeType<'input>,
          T: ParseTreeListener<'input, Node> + 'a + ?Sized,
          Node::Type: Listenable<'input, T> {
    // #[doc(hidden)]
    // pub fn new() -> Self{ Self(PhantomData) }

    pub fn walk<Listener, Ctx>(listener: Box<Listener>, t: &Ctx) -> Box<Listener>
        where
            Box<Listener>: CoerceUnsized<Box<T>>,
            for<'x> &'x Ctx: CoerceUnsized<&'x Node::Type>
    {
        let mut listener = listener as Box<T>;
        Self::walk_inner(&mut listener, t as &Node::Type);

        // just cast back
        unsafe { Box::<Listener>::from_raw(Box::into_raw(listener) as *mut _) }
    }

    fn walk_inner(listener: &mut Box<T>, t: &Node::Type) {
        // if t.self_id() == ErrorNode::<'a, Ctx::TF>::id() {
        //     let err = cast::<_, ErrorNode::<'a, Ctx::TF>>(t);
        //     listener.visit_error_node(err);
        //     return;
        // }
        // if t.self_id() == TerminalNode::<'a, Ctx::TF>::id() {
        //     let leaf = cast::<_, TerminalNode::<'a, Ctx::TF>>(t);
        //     listener.visit_terminal(leaf);
        //     return;
        // }

        // listener.enter_every_rule(t);
        t.enter(listener);

        for child in t.get_children() {
            Self::walk_inner(listener, child.deref())
        }

        t.exit(listener);
        // listener.exit_every_rule(t);
    }
}
