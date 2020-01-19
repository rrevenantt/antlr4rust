use std::any::Any;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::sync::Arc;

use crate::atn::ATN;
use crate::atn_simulator::IATNSimulator;
use crate::common_token_factory::TokenFactory;
use crate::error_listener::{ConsoleErrorListener, ErrorListener, ProxyErrorListener};
use crate::error_strategy::ErrorStrategy;
use crate::errors::{ANTLRError, RecognitionError};
use crate::int_stream::IntStream;
use crate::interval_set::IntervalSet;
use crate::parser_atn_simulator::ParserATNSimulator;
use crate::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, ParserRuleContextType};
use crate::recognizer::{Actions, Recognizer};
use crate::rule_context::RuleContext;
use crate::token::{OwningToken, Token, TOKEN_EOF};
use crate::token_source::TokenSource;
use crate::token_stream::TokenStream;
use crate::tree::{ErrorNode, ErrorNodeCtx, ParseTreeListener, TerminalNode, TerminalNodeCtx};
use crate::vocabulary::{Vocabulary, VocabularyImpl};

pub trait Parser: Recognizer {
    fn get_interpreter(&self) -> &ParserATNSimulator;

    fn get_token_factory(&self) -> &dyn TokenFactory;
    fn get_parser_rule_context(&self) -> &Rc<dyn ParserRuleContext>;
    //    fn set_parser_rule_context(&self, v: ParserRuleContext);
    fn consume(&mut self, err_handler: &mut dyn ErrorStrategy);
    //    fn get_parse_listeners(&self) -> Vec<ParseTreeListener>;
    //fn sempred(&mut self, _localctx: Option<&dyn ParserRuleContext>, rule_index: isize, action_index: isize) -> bool { true }

    fn precpred(&self, localctx: Option<&dyn ParserRuleContext>, precedence: isize) -> bool;

    //    fn get_error_handler(&self) -> ErrorStrategy;
//    fn set_error_handler(&self, e: ErrorStrategy);
    fn get_input_stream_mut(&mut self) -> &mut dyn TokenStream;
    fn get_input_stream(&self) -> &dyn TokenStream;
    fn get_current_token(&self) -> &dyn Token;
    fn get_expected_tokens(&self) -> IntervalSet;

    fn add_error_listener(&mut self, listener: Box<dyn ErrorListener>);
    fn notify_error_listeners(&self, msg: String, offending_token: Option<isize>, err: Option<&ANTLRError>);
    fn get_error_lister_dispatch<'a>(&'a self) -> Box<dyn ErrorListener + 'a>;

    fn is_expected_token(&self, symbol: isize) -> bool;
    fn get_precedence(&self) -> isize;

    fn get_state(&self) -> isize;
    fn set_state(&mut self, v: isize);
    fn get_rule_invocation_stack(&self) -> Vec<String>;
}

/// helper trait to be able to extend listener behavior
pub trait ListenerCaller<T: ParseTreeListener + ?Sized + 'static = dyn ParseTreeListener> {
    fn enter_rule(ctx: &dyn ParserRuleContext, listener: &mut Box<T>) {}
    fn exit_rule(ctx: &dyn ParserRuleContext, listener: &mut Box<T>) {}
}

pub struct ListenerCallerDefault;

impl ListenerCaller for ListenerCallerDefault {}

pub struct BaseParser<
    Ext: ParserRecog<Recog=Self> + 'static,
    T: ParseTreeListener + ?Sized + 'static = dyn ParseTreeListener,
    L: ListenerCaller<T> + 'static = ListenerCallerDefault> {
    interp: Arc<ParserATNSimulator>,
    pub ctx: Option<ParserRuleContextType>,
    // if `build_parse_trees` is false
//    ctx_owner:Vec<Rc<dyn ParserRuleContext>>,
    pub build_parse_trees: bool,
    pub matched_eof: bool,

    state: isize,
    pub input: Box<dyn TokenStream>,
    precedence_stack: Vec<isize>,

    //    tracer: * TraceListener,
    parse_listeners: Vec<Box<T>>,
    _syntax_errors: Cell<isize>,
    error_listeners: RefCell<Vec<Box<dyn ErrorListener>>>,
//    vocabulary:Box<dyn Vocabulary>,

    ext: Ext,
    phantom: PhantomData<L>,
}

impl<T, L, Ext> Deref for BaseParser<Ext, T, L>
    where T: ParseTreeListener + ?Sized + 'static, L: ListenerCaller<T> + 'static, Ext: ParserRecog<Recog=Self> + 'static {
    type Target = Ext;

    fn deref(&self) -> &Self::Target {
        &self.ext
    }
}

impl<T, L, Ext> DerefMut for BaseParser<Ext, T, L>
    where T: ParseTreeListener + ?Sized + 'static, L: ListenerCaller<T> + 'static, Ext: ParserRecog<Recog=Self> + 'static {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ext
    }
}


pub trait ParserRecog: Recognizer + Actions {}

impl<T, L, Ext> Recognizer for BaseParser<Ext, T, L>
    where T: ParseTreeListener + ?Sized + 'static, L: ListenerCaller<T> + 'static, Ext: ParserRecog<Recog=Self> + 'static {
    fn sempred(&mut self, localctx: &dyn ParserRuleContext, rule_index: isize, action_index: isize) -> bool {
        <Ext as Actions>::sempred(localctx, rule_index, action_index, self)
    }

    fn get_rule_names(&self) -> &[&str] {
        self.ext.get_rule_names()
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        self.ext.get_vocabulary()
    }

    fn get_grammar_file_name(&self) -> &str {
        self.ext.get_grammar_file_name()
    }

    fn get_atn(&self) -> &ATN {
        self.interp.atn()
    }
}

impl<T, L, Ext> Parser for BaseParser<Ext, T, L>
    where T: ParseTreeListener + ?Sized + 'static, L: ListenerCaller<T> + 'static, Ext: ParserRecog<Recog=Self> + 'static {
    fn get_interpreter(&self) -> &ParserATNSimulator {
        self.interp.as_ref()
    }

    fn get_token_factory(&self) -> &dyn TokenFactory {
        self.input.get_token_source().get_token_factory()
    }

    fn get_parser_rule_context(&self) -> &Rc<dyn ParserRuleContext> {
        self.ctx.as_ref().unwrap()
    }


    fn consume(&mut self, err_handler: &mut dyn ErrorStrategy) {
        let o = self.get_current_token().to_owned();
        if o.get_token_type() != TOKEN_EOF {
            self.input.consume();
        }
        if self.build_parse_trees || !self.parse_listeners.is_empty() {
            if err_handler.in_error_recovery_mode(self) {
                let node = self.ctx.as_deref().unwrap().add_error_node(self.create_error_node(o));
                let node = unsafe { &*(node.deref() as *const dyn ParserRuleContext as *const ErrorNode) };
                for listener in &self.parse_listeners {
                    listener.visit_error_node(node)
                }
            } else {
                let node = self.ctx.as_deref().unwrap().add_token_node(self.create_token_node(o));
                let node = unsafe { &*(node.deref() as *const dyn ParserRuleContext as *const TerminalNode) };
                for listener in &self.parse_listeners {
                    listener.visit_terminal(node)
                }
            }
        }
    }

    fn precpred(&self, localctx: Option<&dyn ParserRuleContext>, precedence: isize) -> bool {
//        localctx.map(|it|println!("check at{}",it.to_string_tree(self)));
//        println!("{}",self.get_precedence());
        precedence >= self.get_precedence()
    }

    fn get_input_stream_mut(&mut self) -> &mut dyn TokenStream {
        self.input.as_mut()
    }

    fn get_input_stream(&self) -> &dyn TokenStream {
        self.input.as_ref()
    }

    fn get_current_token(&self) -> &dyn Token {
        self.input.get(self.input.index())
    }

//    fn get_error_handler(&self) -> _ {
//        unimplemented!()
//    }
//
//    fn set_error_handler(&self, e: _) {
//        unimplemented!()
//    }

    fn get_expected_tokens(&self) -> IntervalSet {
        self.interp.atn().get_expected_tokens(self.state, self.ctx.as_ref().unwrap())
    }

    fn add_error_listener(&mut self, listener: Box<dyn ErrorListener>) {
        self.error_listeners.borrow_mut().push(listener)
    }

    fn notify_error_listeners(&self, msg: String, offending_token: Option<isize>, err: Option<&ANTLRError>) {
        self._syntax_errors.update(|it| it + 1);
        let offending_token = match offending_token {
            None => Some(self.get_current_token()),
            Some(x) => Some(self.input.get(x)),
        };
        let line = offending_token.map(|x| x.get_line()).unwrap_or(-1);
        let column = offending_token.map(|x| x.get_column()).unwrap_or(-1);

        for listener in self.error_listeners.borrow().iter() {
            listener.syntax_error(self, offending_token, line, column, &msg, err)
        }
    }

    fn get_error_lister_dispatch<'a>(&'a self) -> Box<dyn ErrorListener + 'a> {
        Box::new(ProxyErrorListener { delegates: self.error_listeners.borrow() })
    }

    fn is_expected_token(&self, symbol: isize) -> bool {
        unimplemented!()
    }

    fn get_precedence(&self) -> isize {
        *self.precedence_stack.last().unwrap_or(&-1)
    }

    fn get_state(&self) -> isize {
        self.state
    }

    fn set_state(&mut self, v: isize) {
        self.state = v;
    }

    fn get_rule_invocation_stack(&self) -> Vec<String> {
        let mut vec = Vec::new();
        let rule_names = self.get_rule_names();
        let mut ctx = self.get_parser_rule_context().clone();
        loop {
            let rule_index = ctx.get_rule_index();
            vec.push(if rule_index < 0 { "n/a".to_owned() } else { rule_names[rule_index].to_owned() });
            ctx = if let Some(parent) = ctx.get_parent_ctx() {
                parent
            } else {
                break
            }
        }
        vec
    }

//    fn get_rule_invocation_stack(&self, c: _) -> Vec<String> {
//        unimplemented!()
//    }
}

impl<T, L, Ext> BaseParser<Ext, T, L>
    where T: ParseTreeListener + ?Sized + 'static, L: ListenerCaller<T> + 'static, Ext: ParserRecog<Recog=Self> + 'static {
    pub fn new_base_parser(
        input: Box<dyn TokenStream>,
        interpreter: Arc<ParserATNSimulator>,
        ext: Ext,
    ) -> BaseParser<Ext, T, L> {
        BaseParser {
//            base: BaseRecognizer::new_base_recognizer(),
            interp: interpreter,
            ctx: None,
            build_parse_trees: true,
            matched_eof: false,
            state: -1,
            input,
            precedence_stack: vec![0],
            parse_listeners: vec![],
            _syntax_errors: Cell::new(0),
            error_listeners: RefCell::new(vec![Box::new(ConsoleErrorListener {})]),
//            vocabulary: VocabularyImpl::from_token_names(&[]),
//            vocabulary:Box::new(vocabulary),
            ext,
            phantom: Default::default(),
        }
    }

//
//    fn reset(&self) { unimplemented!() }

    pub fn match_token(&mut self, ttype: isize, err_handler: &mut dyn ErrorStrategy) -> Result<OwningToken, ANTLRError> {
        let mut token = self.get_current_token().to_owned();
        if token.get_token_type() == ttype {
            if ttype == TOKEN_EOF { self.matched_eof = true; }

            err_handler.report_match(self);
//        println!("successfully consumed: {}, index {}",ttype,self.input.index());
            self.consume(err_handler);
        } else {
//        println!("failed to match, expected {}, got {}, index {}",ttype,token,self.input.index());
            token = err_handler.recover_inline(self)?;
            if self.build_parse_trees && token.get_token_index() == -1 {
                self.ctx.as_ref().unwrap().add_error_node(self.create_error_node(token.clone()));
            }
        }
        return Ok(token);
    }

    pub fn match_wildcard(&mut self, err_handler: &mut dyn ErrorStrategy) -> Result<OwningToken, ANTLRError> {
        let mut t = self.get_current_token().to_owned();
        if t.get_token_type() > 0 {
            err_handler.report_match(self);
            self.consume(err_handler);
        } else {
            t = err_handler.recover_inline(self)?;
            if self.build_parse_trees && t.get_token_index() == -1 {
                self.ctx.as_ref().unwrap().add_error_node(self.create_error_node(t.clone()));
            }
        }
        return Ok(t);
    }

    pub fn add_parse_listener(&mut self, listener: Box<T>) -> usize {
        self.parse_listeners.push(listener);
        self.parse_listeners.len() - 1
    }

    //todo, looks like we need to return some kind of handler to be able to remove one listener
    pub fn remove_parse_listener(&self, listener_id: usize) { unimplemented!() }

    pub fn remove_parse_listeners(&mut self) { self.parse_listeners.clear() }

    pub fn trigger_enter_rule_event(&mut self) {
        for listener in &mut self.parse_listeners {
            L::enter_rule(self.ctx.as_deref().unwrap(), listener);
        }
    }

    pub fn trigger_exit_rule_event(&mut self) {
        for listener in self.parse_listeners.iter_mut().rev() {
            L::exit_rule(self.ctx.as_deref().unwrap(), listener);
        }
    }
//
//    fn set_token_factory(&self, factory: TokenFactory) { unimplemented!() }
//
//
//    fn get_atn_with_bypass_alts(&self) { unimplemented!() }
//
//    fn compile_parse_tree_pattern(&self, pattern, patternRuleIndex: Lexer, lexer: Lexer) { unimplemented!() }
//
//    fn set_input_stream(&self, input: TokenStream) { unimplemented!() }
//
//    fn set_token_stream(&self, input: TokenStream) { unimplemented!() }

    fn add_context_to_parse_tree(&mut self) {
        let parent = self.ctx.as_ref().unwrap()
            .get_parent_ctx();

        if let Some(parent) = parent {
            parent.add_child(self.ctx.clone().unwrap())
        }
    }

    pub fn enter_rule(&mut self, localctx: Rc<dyn ParserRuleContext>, state: isize, rule_index: usize) {
        self.set_state(state);
        localctx.set_start(self.input.lt(1).map(Token::to_owned));
        self.ctx = Some(localctx);
//        let mut localctx = Rc::get_mut(self.ctx.as_mut().unwrap()).unwrap();
        if self.build_parse_trees {
            self.add_context_to_parse_tree()
        }
        self.trigger_enter_rule_event();
    }

    pub fn exit_rule(&mut self) {
        if self.matched_eof {
            self.ctx.as_ref().unwrap().set_stop(self.input.lt(1).map(Token::to_owned))
        } else {
            self.ctx.as_ref().unwrap().set_stop(self.input.lt(-1).map(Token::to_owned))
        }
        self.trigger_exit_rule_event();
        self.set_state(self.get_parser_rule_context().get_invoking_state());
//        let mut currctx = &mut self.ctx;
//        let mut ctx = currctx.as_mut().unwrap();
        let mut parent = self.ctx.as_ref().unwrap().get_parent_ctx();
        mem::replace(&mut self.ctx, parent);
    }

    // todo make new_ctx not option
    pub fn enter_outer_alt(&mut self, new_ctx: Option<Rc<dyn ParserRuleContext>>, alt_num: isize) {
        if let Some(new_ctx) = new_ctx {
            new_ctx.set_alt_number(alt_num);

            let ctx = self.ctx.as_ref().unwrap();
            if self.build_parse_trees && self.ctx.is_some() && !Rc::ptr_eq(&new_ctx, ctx) {
                if let Some(parent) = ctx.get_parent_ctx() {
                    parent.remove_last_child();
                    parent.add_child(new_ctx.clone())
                }
            }

            self.ctx = Some(new_ctx)
        }
    }


    pub fn enter_recursion_rule(&mut self, localctx: Rc<dyn ParserRuleContext>, state: isize, rule_index: usize, precedence: isize) {
        self.set_state(state);
        self.precedence_stack.push(precedence);
        localctx.set_start(self.input.lt(1).map(Token::to_owned));
        //println!("{}",self.input.lt(1).map(Token::to_owned).unwrap());
        self.ctx = Some(localctx);
        self.trigger_enter_rule_event()
    }

    pub fn push_new_recursion_context(&mut self, localctx: Rc<dyn ParserRuleContext>, state: isize, rule_index: usize) {
        let prev = self.ctx.take().unwrap();
        prev.set_parent(&Some(localctx.clone()));
        prev.set_invoking_state(state);
        prev.set_stop(self.input.lt(-1).map(Token::to_owned));

//        println!("{}",prev.get_start().unwrap());
        localctx.set_start(prev.get_start());
        self.ctx = Some(localctx);

        if self.build_parse_trees {
            self.ctx.as_ref().unwrap().add_child(prev);
        }
        self.trigger_enter_rule_event();
//        unimplemented!()
    }

    pub fn unroll_recursion_context(&mut self, parent_ctx: Option<Rc<dyn ParserRuleContext>>) {
        self.precedence_stack.pop();
        let retctx = self.ctx.clone().unwrap();
        retctx.set_stop(self.input.lt(-1).map(Token::to_owned));
        if !self.parse_listeners.is_empty() {
            while !Rc::ptr_eq(self.ctx.as_ref().unwrap(), parent_ctx.as_ref().unwrap()) {
//                println!("{:p} {:p}",self.ctx.as_deref().unwrap(),parent_ctx.as_ref().unwrap());
                self.trigger_exit_rule_event();
                self.ctx = self.ctx.as_ref().unwrap().get_parent_ctx()
            }
        } else {
            self.ctx = parent_ctx;
        }

        //self.ctx is now parent
        retctx.set_parent(&self.ctx);

//        println!("{:?}",self.ctx.as_ref().map(|it|it.to_string_tree(self)));
        if self.build_parse_trees && self.ctx.is_some() {
            self.ctx.as_ref().unwrap().add_child(retctx);
        }
    }

    fn create_token_node(&self, token: OwningToken) -> TerminalNode {
        BaseParserRuleContext::new_parser_ctx(None, -1, TerminalNodeCtx { symbol: token })
    }

    fn create_error_node(&self, token: OwningToken) -> ErrorNode {
        BaseParserRuleContext::new_parser_ctx(None, -1, ErrorNodeCtx(TerminalNodeCtx { symbol: token }))
    }

    pub fn dump_dfa(&self) {
        let mut seen_one = false;
        for dfa in self.interp.decision_to_dfa() {
            // because s0 is saved in dfa for Rust version
            if dfa.states.read().unwrap().len() > 1 + (dfa.is_precedence_dfa() as usize) {
                if seen_one { println!() }
                println!("Decision {}:", dfa.decision);
                print!("{}", dfa.to_string(self.get_vocabulary()));
                seen_one = true;
            }
        }
    }

    //    fn get_invoking_context(&self, ruleIndex: isize) -> ParserRuleContext { unimplemented!() }
//
//    fn in_context(&self, context: ParserRuleContext) -> bool { unimplemented!() }
//
//    fn get_expected_tokens_within_current_rule(&self) -> * IntervalSet { unimplemented!() }
//
//
//    fn get_rule_index(&self, ruleName: String) -> int { unimplemented!() }
//
//    fn get_dfaStrings(&self) -> String { unimplemented!() }
//
//    fn get_source_name(&self) -> String { unimplemented!() }
//
//    fn set_trace(&self, trace: * TraceListener) { unimplemented!() }
}