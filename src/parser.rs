use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::sync::Arc;

use crate::atn::ATN;
use crate::atn_simulator::IATNSimulator;
use crate::common_token_factory::TokenFactory;
use crate::error_listener::{ConsoleErrorListener, ErrorListener};
use crate::error_strategy::ErrorStrategy;
use crate::errors::{ANTLRError, RecognitionError};
use crate::int_stream::IntStream;
use crate::interval_set::IntervalSet;
use crate::parser_atn_simulator::ParserATNSimulator;
use crate::parser_rule_context::ParserRuleContext;
use crate::recognizer::Recognizer;
use crate::rule_context::RuleContext;
use crate::token::{Token, TOKEN_EOF};
use crate::token_source::TokenSource;
use crate::token_stream::TokenStream;
use crate::tree::{ListenerDispatch, ParseTreeListener};
use crate::vocabulary::{Vocabulary, VocabularyImpl};

pub trait Parser: Recognizer {
    fn get_interpreter(&self) -> &ParserATNSimulator;
//    fn get_vocabulary(&self) -> &dyn Vocabulary;

    fn get_token_factory(&self) -> &dyn TokenFactory;
    fn get_parser_rule_context(&self) -> &Rc<ParserRuleContext>;
    //    fn set_parser_rule_context(&self, v: ParserRuleContext);
    fn consume(&mut self);
//    fn get_parse_listeners(&self) -> Vec<ParseTreeListener>;

    //    fn get_error_handler(&self) -> ErrorStrategy;
//    fn set_error_handler(&self, e: ErrorStrategy);
    fn get_input_stream(&mut self) -> &mut dyn TokenStream;
    fn get_current_token(&mut self) -> &dyn Token;
    fn get_expected_tokens(&self) -> IntervalSet;
    fn notify_error_listeners(&mut self, msg: String, offending_token: Option<isize>, err: Option<&ANTLRError>);
    fn is_expected_token(&self, symbol: isize) -> bool;
    fn get_precedence(&self) -> isize;

    fn get_state(&self) -> isize;
    fn set_state(&mut self, v: isize);
//    fn get_rule_invocation_stack(&self, c: ParserRuleContext) -> Vec<String>;
}

/// helper trait to be able to extend listener behavior
pub trait ListenerCaller<T: ParseTreeListener + ?Sized = dyn ParseTreeListener> {
    fn enter_rule(ctx: &dyn ParserRuleContext, listener: &mut T) {}
    fn exit_rule(ctx: &dyn ParserRuleContext, listener: &mut T) {}
}

pub struct ListenerCallerDefault;

impl ListenerCaller for ListenerCallerDefault {}

pub struct RecogDefault;

impl Recognizer for RecogDefault {}

pub struct BaseParser<
    Ext: Recognizer + 'static,
    T: ParseTreeListener + ?Sized = dyn ParseTreeListener,
    L: ListenerCaller<T> + 'static = ListenerCallerDefault> {
    interp: Arc<ParserATNSimulator>,
    pub ctx: Option<Rc<dyn ParserRuleContext>>,
    // if `build_parse_trees` is false
//    ctx_owner:Vec<Rc<dyn ParserRuleContext>>,
    build_parse_trees: bool,
    pub matched_eof: bool,

    state: isize,
    pub input: Box<dyn TokenStream>,
    precedence_stack: Vec<isize>,

    //    tracer: * TraceListener,
    parse_listeners: Vec<Box<T>>,
    _syntax_errors: isize,
    error_listeners: RefCell<Vec<Box<dyn ErrorListener>>>,
//    vocabulary:Box<dyn Vocabulary>,

    ext: Ext,
    phantom: PhantomData<L>
}

impl<T: ParseTreeListener + ?Sized, L: ListenerCaller<T> + 'static, Ext: Recognizer + 'static> Recognizer for BaseParser<Ext, T, L> {
    fn get_rule_names(&self) -> &[&str] {
        self.ext.get_rule_names()
    }

    fn get_vocabulary(&self) -> &Vocabulary {
        self.ext.get_vocabulary()
    }

    fn get_grammar_file_name(&self) -> &str {
        self.ext.get_grammar_file_name()
    }

    fn get_atn(&self) -> &ATN {
        self.interp.atn()
    }
}

impl<T: ParseTreeListener + ?Sized, L: ListenerCaller<T> + 'static, Ext: Recognizer + 'static> Parser for BaseParser<Ext, T, L> {
    fn get_interpreter(&self) -> &ParserATNSimulator {
        self.interp.as_ref()
    }

    fn get_token_factory(&self) -> &dyn TokenFactory {
        unimplemented!()
    }

    fn get_parser_rule_context(&self) -> &Rc<dyn ParserRuleContext> {
        self.ctx.as_ref().unwrap()
    }


    fn consume(&mut self) {
        if self.input.la(1) != TOKEN_EOF {
            self.input.consume();
        }
        if self.build_parse_trees {
            unreachable!();
//            let a:RangeInclusiveEquality;
        }
//        self.input.lt(-1)
    }

    fn get_input_stream(&mut self) -> &mut dyn TokenStream {
        self.input.as_mut()
    }

    fn get_current_token(&mut self) -> &Token {
        self.input.lt(1)
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

    fn notify_error_listeners(&mut self, msg: String, offending_token: Option<isize>, err: Option<&ANTLRError>) {
        self._syntax_errors += 1;
        let offending_token = match offending_token {
            None => None,
            Some(x) => Some(self.input.get(x)),
        };
        let line = offending_token.map(|x| x.get_line()).unwrap_or(-1);
        let column = offending_token.map(|x| x.get_column()).unwrap_or(-1);

        for listener in self.error_listeners.borrow_mut().iter_mut() {
            listener.syntax_error(self, offending_token, line, column, &msg, err)
        }
    }

    fn is_expected_token(&self, symbol: isize) -> bool {
        unimplemented!()
    }

    fn get_precedence(&self) -> isize {
        *self.precedence_stack.last().unwrap()
    }

    fn get_state(&self) -> isize {
        self.state
    }

    fn set_state(&mut self, v: isize) {
        self.state = v;
    }

//    fn get_rule_invocation_stack(&self, c: _) -> Vec<String> {
//        unimplemented!()
//    }
}

impl<T: ParseTreeListener + ?Sized, L: ListenerCaller<T> + 'static, Ext: Recognizer + 'static> BaseParser<Ext, T, L> {
    pub fn new_base_parser(
        input: Box<dyn TokenStream>,
        interpreter: Arc<ParserATNSimulator>,
        ext: Ext,
    ) -> BaseParser<Ext, T, L> {
        BaseParser {
//            base: BaseRecognizer::new_base_recognizer(),
            interp: interpreter,
            ctx: None,
            build_parse_trees: false,
            matched_eof: false,
            state: 0,
            input,
            precedence_stack: vec![0],
            parse_listeners: vec![],
            _syntax_errors: 0,
            error_listeners: RefCell::new(vec![Box::new(ConsoleErrorListener {})]),
//            vocabulary: VocabularyImpl::from_token_names(&[]),
//            vocabulary:Box::new(vocabulary),
            ext,
            phantom: Default::default()
        }
    }

//
//    fn reset(&self) { unimplemented!() }
//
pub fn match_token(&mut self, ttype: isize, err_handler: &mut dyn ErrorStrategy) -> Result<&dyn Token, ANTLRError> {
        let token = self.input.la(1);
        if token == ttype {
            if ttype == TOKEN_EOF { self.matched_eof = true; }

            err_handler.report_match(self);
//        println!("successfully consumed: {}, index {}",ttype,self.input.index());
            self.consume();
        } else {
//        println!("failed to match, expected {}, got {}, index {}",ttype,token,self.input.index());
            let t = err_handler.recover_inline(self)?;
            if self.build_parse_trees && t.get_token_type() == -1 {
                unimplemented!()
            }
        }
    return Ok(self.input.lt(1));
}

    pub fn match_wildcard(&mut self, err_handler: &mut dyn ErrorStrategy) -> Result<(), ANTLRError> {
        let t = self.get_current_token();
        if t.get_token_type() > 0 {
            err_handler.report_match(self);
            self.consume();
        } else {
            let t = err_handler.recover_inline(self)?;
            if self.build_parse_trees && t.get_token_type() == -1 {
                unimplemented!()
            }
        }
        return Ok(())
    }

    pub fn add_parse_listener(&mut self, listener: Box<T>) -> usize {
        self.parse_listeners.push(listener);
        self.parse_listeners.len() - 1
    }

    //todo, looks like we need hashset to be able to remove one listener
    pub fn remove_parse_listener(&self, listener_id: usize) { unimplemented!() }

    pub fn remove_parse_listeners(&mut self) { self.parse_listeners.clear() }

    fn trigger_enter_rule_event(&mut self) {
        for listener in &mut self.parse_listeners {
            L::enter_rule(self.ctx.as_deref().unwrap(), listener);
        }
    }

    fn trigger_exit_rule_event(&mut self) {
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

    fn add_context_to_parse_tree(&self) { unimplemented!() }


    pub fn enter_rule(&mut self, localctx: Rc<dyn ParserRuleContext>, state: isize, rule_index: usize) {
        self.set_state(state);
        self.ctx = Some(localctx);
        let mut localctx = Rc::get_mut(self.ctx.as_mut().unwrap()).unwrap();
        localctx.set_start(self.input.lt(1).get_token_index());
        if self.build_parse_trees {
            unimplemented!()
        }
        self.trigger_enter_rule_event();
    }

    pub fn exit_rule(&mut self) {
        self.trigger_exit_rule_event();
        self.set_state(self.get_parser_rule_context().get_invoking_state());
        let mut localctx = &mut self.ctx;
        let mut ctx = localctx.as_mut().unwrap();
        let mut parent = ctx.get_parent_ctx()
            .as_ref()
            .map(|it| it
                .upgrade()
                .unwrap()
            );
        mem::replace(localctx, parent);
    }

    pub fn enter_outer_alt(&mut self, new_ctx: Option<Rc<dyn ParserRuleContext>>, altNum: isize) {
        new_ctx.map(|it| self.ctx = Some(it));
    }


    fn enter_recursion_rule(&mut self, localctx: Rc<dyn ParserRuleContext>, state: isize, ruleIndex: isize, precedence: isize) {
        self.set_state(state);
        self.precedence_stack.push(precedence);
        self.ctx = Some(localctx);
        self.trigger_enter_rule_event()
    }

//    fn push_new_recursion_context(&self, localctx: ParserRuleContext, state: isize, ruleIndex: isize) { unimplemented!() }

    fn unroll_recursion_contexts(&mut self/*, parentCtx: ParserRuleContext*/) {
        self.precedence_stack.pop();
    }

//    fn get_invoking_context(&self, ruleIndex: isize) -> ParserRuleContext { unimplemented!() }
//
//    fn precpred(&self, localctx: RuleContext, precedence: isize) -> bool { unimplemented!() }
//
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
//    fn dump_dfa(&self) { unimplemented!() }
//
//    fn get_source_name(&self) -> String { unimplemented!() }
//
//    fn set_trace(&self, trace: * TraceListener) { unimplemented!() }
}