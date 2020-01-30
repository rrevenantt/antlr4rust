use std::any::{Any, type_name, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::fmt::{Debug, Error, Formatter};
use std::ops::{CoerceUnsized, Deref, DerefMut};
use std::rc::Rc;

use crate::errors::ANTLRError;
use crate::interval_set::Interval;
use crate::parser::{ListenerCaller, Parser, ParserRecog};
use crate::recognizer::Recognizer;
use crate::rule_context::{BaseRuleContext, CustomRuleContext, EmptyCustomRuleContext, RuleContext};
use crate::token::{OwningToken, Token};
use crate::tree::{ErrorNode, ErrorNodeCtx, ParseTree, ParseTreeListener, TerminalNode, TerminalNodeCtx, Tree};
use crate::trees;

pub trait ParserRuleContext: RuleContext + CustomRuleContext + ParseTree + Any + Debug {
    fn set_exception(&self, e: ANTLRError);

    fn set_start(&self, t: Option<OwningToken>);
    fn get_start(&self) -> Option<OwningToken>;

    fn set_stop(&self, t: Option<OwningToken>);
    fn get_stop(&self) -> Option<OwningToken>;


    fn add_token_node(&self, token: TerminalNode) -> Rc<dyn ParserRuleContext>;
    fn add_error_node(&self, bad_token: ErrorNode) -> Rc<dyn ParserRuleContext>;

    fn add_child(&self, child: ParserRuleContextType);
    fn remove_last_child(&self);

    fn enter_rule(&self, listener: &mut dyn Any);
    fn exit_rule(&self, listener: &mut dyn Any);

    fn child_of_type<T: ParserRuleContext>(&self, pos: usize) -> Rc<T> where Self: Sized {
        let result = self.get_children().iter()
            .filter(|&it| it.deref().type_id() == TypeId::of::<T>())
            .nth(pos).unwrap()
            .clone();

        cast_rc(result)
    }

    fn children_of_type<T: ParserRuleContext>(&self) -> Vec<Rc<T>> where Self: Sized {
        self.get_children()
            .iter()
            .filter(|&it| it.deref().type_id() == TypeId::of::<T>())
            .map(|it| cast_rc::<T>(it.clone()))
            .collect()
    }

    fn get_token(&self, ttype: isize, pos: usize) -> Rc<TerminalNode> {
        self.get_children()
            .iter()
            .filter(|&it| it.deref().type_id() == TypeId::of::<TerminalNode>())
            .map(|it| cast_rc::<TerminalNode>(it.clone()))
            .filter(|it| it.symbol.get_token_type() == ttype)
            .nth(pos).unwrap()
    }

    fn get_tokens(&self, _ttype: isize) -> Box<dyn Iterator<Item=&OwningToken>> {
        unimplemented!()
    }

    fn upcast_any(&self) -> &dyn Any;

    fn upcast(&self) -> &dyn ParserRuleContext;
}

impl dyn ParserRuleContext {
    fn to_string(self: &Rc<Self>, rule_names: Option<&[&str]>, stop: Option<Rc<dyn ParserRuleContext>>) -> String {
        let mut result = String::from("[");
        let mut next: Option<Rc<dyn ParserRuleContext>> = Some(self.clone());
        while let Some(ref p) = next {
            if stop.is_some() && (stop.is_none() || Rc::ptr_eq(p, stop.as_ref().unwrap())) { break }


            if let Some(rule_names) = rule_names {
                let rule_index = p.get_rule_index();
                let rule_name = rule_names.get(rule_index).map(|&it| it.to_owned())
                    .unwrap_or_else(|| rule_index.to_string());
                result.extend(rule_name.chars());
                result.push(' ');
            } else {
                if !p.is_empty() {
                    result.extend(p.get_invoking_state().to_string().chars());
                    result.push(' ');
                }
            }

            next = p.get_parent().clone();
        }
        // not optimal but we don't care here
        if result.chars().last() == Some(' ') {
            result.pop();
        }

        result.push(']');
        return result
    }
}


//requires ParserRuleContext to be Sync
//lazy_static! {
//    pub static ref EMPTY_CTX: Box<dyn ParserRuleContext> =
//        Box::new(BaseParserRuleContext::new_parser_ctx(None,-1,CustomRuleContextInternal));
//}

pub type LexerContext = BaseParserRuleContext<EmptyCustomRuleContext>;

//todo do not calc this every time, maybe threadlocal? or it might be ok as it is because it is inlined
#[inline]
pub(crate) fn empty_ctx() -> Box<dyn ParserRuleContext> {
    Box::new(BaseParserRuleContext::new_parser_ctx(None, -1, EmptyCustomRuleContext))
}

#[inline]
fn cast_rc<T: ParserRuleContext>(ctx: Rc<dyn ParserRuleContext>) -> Rc<T> {
    // not sure how safe it is
    unsafe { Rc::from_raw(Rc::into_raw(ctx) as *const T) }
}

#[inline]
pub fn cast<T: ParserRuleContext + ?Sized, Result>(ctx: &T) -> &Result {
    unsafe { &*(ctx as *const T as *const Result) }
}

/// should be called from generated parser only
#[inline]
pub fn cast_mut<T: ParserRuleContext + ?Sized, Result>(ctx: &mut Rc<T>) -> &mut Result {
//    if Rc::strong_count(ctx) != 1 { panic!("cant mutate Rc with multiple strong ref count"); }
// is it safe because parser does not save/move mutable references anywhere.
// they are only used to write data immediately in the corresponding expression
    unsafe { &mut *(Rc::get_mut_unchecked(ctx) as *mut T as *mut Result) }
}


pub type ParserRuleContextType = Rc<dyn ParserRuleContext>;

pub struct BaseParserRuleContext<Ctx: CustomRuleContext> {
    base: BaseRuleContext<Ctx>,

    start: RefCell<Option<OwningToken>>,
    stop: RefCell<Option<OwningToken>>,
    exception: Option<Box<ANTLRError>>,
    /// List of children of current node
    pub(crate) children: RefCell<Vec<ParserRuleContextType>>,
}

impl<Ctx: CustomRuleContext> Debug for BaseParserRuleContext<Ctx> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(type_name::<Self>())
    }
}

impl<Ctx: CustomRuleContext> RuleContext for BaseParserRuleContext<Ctx> {
    fn get_invoking_state(&self) -> isize {
        self.base.get_invoking_state()
    }

    fn set_invoking_state(&self, t: isize) {
        self.base.set_invoking_state(t)
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
    fn get_rule_index(&self) -> usize { self.base.ext.get_rule_index() }
}

impl<Ctx: CustomRuleContext> Deref for BaseParserRuleContext<Ctx> {
    type Target = Ctx;

    fn deref(&self) -> &Self::Target {
        &self.base.ext
    }
}

impl<Ctx: CustomRuleContext> DerefMut for BaseParserRuleContext<Ctx> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.ext
    }
}

impl<Ctx: CustomRuleContext> Borrow<Ctx> for BaseParserRuleContext<Ctx> {
    fn borrow(&self) -> &Ctx {
        &self.base.ext
    }
}

impl<Ctx: CustomRuleContext> BorrowMut<Ctx> for BaseParserRuleContext<Ctx> {
    fn borrow_mut(&mut self) -> &mut Ctx {
        &mut self.base.ext
    }
}

impl<Ctx: CustomRuleContext> ParserRuleContext for BaseParserRuleContext<Ctx> {
    fn set_exception(&self, _e: ANTLRError) {
        unimplemented!()
//        self.exception = Some(Box::new(e));
    }

    fn set_start(&self, t: Option<OwningToken>) {
        *self.start.borrow_mut() = t;
    }

    fn get_start(&self) -> Option<OwningToken> {
        self.start.borrow().clone()
    }

    fn set_stop(&self, t: Option<OwningToken>) {
        *self.stop.borrow_mut() = t;
    }

    fn get_stop(&self) -> Option<OwningToken> {
        self.stop.borrow().clone()
    }

    fn add_token_node(&self, token: TerminalNode) -> Rc<dyn ParserRuleContext> {
        let node: Rc<dyn ParserRuleContext> = Rc::new(token);
        self.children.borrow_mut().push(node.clone());
        node
    }

    fn add_error_node(&self, bad_token: ErrorNode) -> Rc<dyn ParserRuleContext> {
//        bad_token.base.parent_ctx =
        let node: Rc<dyn ParserRuleContext> = Rc::new(bad_token);
//        Backtrace::new().frames()[0].symbols()[0];

        self.children.borrow_mut().push(node.clone());
        node
    }

    fn add_child(&self, child: ParserRuleContextType) {
        self.children.borrow_mut().push(child);
    }

    fn remove_last_child(&self) {
        self.children.borrow_mut().pop();
    }

    fn enter_rule(&self, listener: &mut dyn Any) {
        Ctx::enter(self, listener)
    }

    fn exit_rule(&self, listener: &mut dyn Any) {
        Ctx::exit(self, listener)
    }

    fn upcast_any(&self) -> &dyn Any {
        self
    }

    fn upcast(&self) -> &dyn ParserRuleContext {
        self
    }
}

impl<Ctx: CustomRuleContext> Tree for BaseParserRuleContext<Ctx> {
    fn get_parent(&self) -> Option<ParserRuleContextType> {
        self.get_parent_ctx()
    }

    fn has_parent(&self) -> bool {
        self.base.parent_ctx.borrow().is_some()
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

    fn get_children(&self) -> Ref<'_, Vec<ParserRuleContextType>> {
        self.children.borrow()
    }

    fn get_children_full(&self) -> &RefCell<Vec<ParserRuleContextType>> {
        &self.children
    }
}

impl<Ctx: CustomRuleContext> ParseTree for BaseParserRuleContext<Ctx> {
    fn get_source_interval(&self) -> Interval {
        unimplemented!()
    }

    fn get_text(&self) -> String {
        unimplemented!()
    }

    fn to_string_tree(&self, r: &dyn Parser) -> String {
        trees::string_tree(self, r.get_rule_names())
    }
}

impl<Ctx: CustomRuleContext> BaseParserRuleContext<Ctx> {
    pub fn new_parser_ctx(parent_ctx: Option<ParserRuleContextType>, invoking_state: isize, ext: Ctx) -> Self {
        BaseParserRuleContext {
            base: BaseRuleContext::new_ctx(parent_ctx, invoking_state, ext),
            start: RefCell::new(None),
            stop: RefCell::new(None),
            exception: None,
            children: RefCell::new(vec![]),
        }
    }
    pub fn copy_from<T: ParserRuleContext + ?Sized>(ctx: &T, ext: Ctx) -> Self {
        BaseParserRuleContext {
            base: BaseRuleContext::new_ctx(ctx.get_parent_ctx(), ctx.get_invoking_state(), ext),
            start: RefCell::new(ctx.get_start().clone()),
            stop: RefCell::new(ctx.get_stop().clone()),
            exception: None,
            children: RefCell::new(ctx.get_children().iter().cloned().collect()),
        }
    }

    pub fn to_string(self: Rc<Self>, rule_names: Option<&[&str]>, stop: Option<Rc<dyn ParserRuleContext>>) -> String {
        (self as Rc<dyn ParserRuleContext>).to_string(rule_names, stop)
    }
}


///////////////////////////////////////////////
// Needed to reduce boilerplate in the generated code,
// because there is no simple way to implement trait for enum
// will not be necessary if some kind of variant types RFC will be merged
//////////////////////////////////////////////
/// workaround trait to overcome conflicting implementations error
pub trait DerefSeal: Deref {}

impl<T: DerefSeal<Target=I> + Debug + 'static, I: ParserRuleContext + ?Sized> ParserRuleContext for T {
    fn set_exception(&self, e: ANTLRError) { self.deref().set_exception(e) }

    fn set_start(&self, t: Option<OwningToken>) { self.deref().set_start(t) }

    fn get_start(&self) -> Option<OwningToken> { self.deref().get_start() }

    fn set_stop(&self, t: Option<OwningToken>) { self.deref().set_stop(t) }

    fn get_stop(&self) -> Option<OwningToken> { self.deref().get_stop() }

    fn add_token_node(&self, token: BaseParserRuleContext<TerminalNodeCtx>) -> Rc<dyn ParserRuleContext> { self.deref().add_token_node(token) }

    fn add_error_node(&self, bad_token: BaseParserRuleContext<ErrorNodeCtx>) -> Rc<dyn ParserRuleContext> { self.deref().add_error_node(bad_token) }

    fn add_child(&self, child: Rc<dyn ParserRuleContext>) { self.deref().add_child(child) }

    fn remove_last_child(&self) { self.deref().remove_last_child() }

    fn enter_rule(&self, listener: &mut dyn Any) { self.deref().enter_rule(listener) }

    fn exit_rule(&self, listener: &mut dyn Any) { self.deref().exit_rule(listener) }

    fn upcast_any(&self) -> &dyn Any { self.deref().upcast_any() }

    fn upcast(&self) -> &dyn ParserRuleContext { self.deref().upcast() }
}

impl<T: DerefSeal<Target=I> + Debug + 'static, I: ParserRuleContext + ?Sized> RuleContext for T {
    fn get_invoking_state(&self) -> isize { self.deref().get_invoking_state() }

    fn set_invoking_state(&self, t: isize) { self.deref().set_invoking_state(t) }

    fn is_empty(&self) -> bool { self.deref().is_empty() }

    fn get_parent_ctx(&self) -> Option<Rc<dyn ParserRuleContext>> { self.deref().get_parent_ctx() }

    fn peek_parent(&self) -> Option<Rc<dyn ParserRuleContext>> { self.deref().peek_parent() }

    fn set_parent(&self, parent: &Option<Rc<dyn ParserRuleContext>>) { self.deref().set_parent(parent) }
}

impl<T: DerefSeal<Target=I> + Debug + 'static, I: ParserRuleContext + ?Sized> ParseTree for T {
    fn get_source_interval(&self) -> Interval { self.deref().get_source_interval() }

    fn get_text(&self) -> String { self.deref().get_text() }

    fn to_string_tree(&self, r: &dyn Parser) -> String { self.deref().to_string_tree(r) }
}

impl<T: DerefSeal<Target=I> + Debug + 'static, I: ParserRuleContext + ?Sized> Tree for T {
    fn get_parent(&self) -> Option<Rc<dyn ParserRuleContext>> { self.deref().get_parent() }

    fn has_parent(&self) -> bool { self.deref().has_parent() }

    fn get_payload(&self) -> Box<dyn Any> { self.deref().get_payload() }

    fn get_child(&self, i: usize) -> Option<Rc<dyn ParserRuleContext>> { self.deref().get_child(i) }

    fn get_child_count(&self) -> usize { self.deref().get_child_count() }

    fn get_children(&self) -> Ref<Vec<Rc<dyn ParserRuleContext>>> { self.deref().get_children() }

    fn get_children_full(&self) -> &RefCell<Vec<Rc<dyn ParserRuleContext>>> { self.deref().get_children_full() }
}

impl<T: DerefSeal<Target=I> + Debug + 'static, I: ParserRuleContext + ?Sized> CustomRuleContext for T {
    fn get_rule_index(&self) -> usize { self.deref().get_rule_index() }

    fn get_alt_number(&self) -> isize { self.deref().get_alt_number() }

    fn set_alt_number(&self, _alt_number: isize) { self.deref().set_alt_number(_alt_number) }
}

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
