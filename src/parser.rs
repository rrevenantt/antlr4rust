use crate::recognizer::Recognizer;
use crate::token_stream::TokenStream;
use crate::common_token_factory::TokenFactory;
use crate::token::{Token, TOKEN_EOF};
use crate::int_stream::IntStream;
use crate::interval_set::IntervalSet;
use crate::errors::{RecognitionError, ANTLRError};
use crate::token_source::TokenSource;
use crate::parser_atn_simulator::ParserATNSimulator;
use crate::error_listener::ErrorListener;
use crate::error_strategy::ErrorStrategy;
use crate::rule_context::RuleContext;
use std::mem;

pub trait Parser {
    fn get_interpreter(&self) -> &mut ParserATNSimulator;

    //    fn get_token_stream<T>(&self) -> &dyn TokenStream<T>;
    fn get_token_factory(&self) -> &dyn TokenFactory;
    //    fn get_parser_rule_context(&self) -> ParserRuleContext;
//    fn set_parser_rule_context(&self, v: ParserRuleContext);
    fn consume(&mut self);
//    fn get_parse_listeners(&self) -> Vec<ParseTreeListener>;

    //    fn get_error_handler(&self) -> ErrorStrategy;
//    fn set_error_handler(&self, e: ErrorStrategy);
    fn get_input_stream(&mut self) -> &mut dyn TokenStream;
    fn get_current_token(&self) -> &dyn Token;
    fn get_expected_tokens(&self) -> IntervalSet;
    fn notify_error_listeners(&self, msg: String, offendingToken: &dyn Token, err: ANTLRError);
    fn is_expected_token(&self, symbol: isize) -> bool;
    fn get_precedence(&self) -> isize;

    fn get_state(&self) -> isize;
    fn set_state(&mut self, v: isize);
//    fn get_rule_invocation_stack(&self, c: ParserRuleContext) -> Vec<String>;
}

pub struct BaseParser {
//    base: BaseRecognizer,

    pub(crate) interpreter: ParserATNSimulator,
    build_parse_trees: bool,
    pub(crate) matched_eof: bool,

    state: isize,
    pub(crate) input: Box<dyn TokenStream>,
//    precedence_stack: IntStack,

    //    tracer: * TraceListener,
//    parse_listeners: Vec<ParseTreeListener>,
    _syntax_errors: isize,
}

impl Parser for BaseParser {
    fn get_state(&self) -> isize {
        self.state
    }

    fn set_state(&mut self, v: isize) {
        self.state = v;
    }


    fn get_interpreter(&self) -> &mut ParserATNSimulator {
        unimplemented!()
    }

    fn get_token_factory(&self) -> &TokenFactory {
        unimplemented!()
    }

    fn consume(&mut self) {
        if self.input.la(1) != TOKEN_EOF {
            self.input.consume();
        }
        if self.build_parse_trees {
            unreachable!()
        }
//        self.input.lt(-1)
    }

//    fn get_error_handler(&self) -> _ {
//        unimplemented!()
//    }
//
//    fn set_error_handler(&self, e: _) {
//        unimplemented!()
//    }

    fn get_input_stream(&mut self) -> &mut dyn TokenStream {
        self.input.as_mut()
    }

    fn get_current_token(&self) -> &Token {
        unimplemented!()
    }

    fn get_expected_tokens(&self) -> IntervalSet {
        unimplemented!()
    }

    fn notify_error_listeners(&self, msg: String, offendingToken: &Token, err: ANTLRError) {
        unimplemented!()
    }

    fn is_expected_token(&self, symbol: isize) -> bool {
        unimplemented!()
    }

    fn get_precedence(&self) -> isize {
        unimplemented!()
    }

//    fn get_rule_invocation_stack(&self, c: _) -> Vec<String> {
//        unimplemented!()
//    }
}

impl BaseParser {
    pub(crate) fn new_base_parser(input: Box<dyn TokenStream>, interpreter: ParserATNSimulator) -> BaseParser {
        BaseParser {
//            base: BaseRecognizer::new_base_recognizer(),
            interpreter,
            build_parse_trees: false,
            matched_eof: false,
            state: 0,
            input,
            _syntax_errors: 0,
        }
    }

    //    fn add_parse_listener(&self, listener: ParseTreeListener) { unimplemented!() }
//
//    fn reset(&self) { unimplemented!() }
//
    pub(crate) fn match_token(&mut self, ttype: isize, err_handler: &mut dyn ErrorStrategy) -> Result<&dyn Token, ANTLRError> {
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
    //
//
//    fn match_wildcard(&self) -> Token { unimplemented!() }
//
//    fn remove_parse_listener(&self, listener: ParseTreeListener) { unimplemented!() }
//
//    fn remove_parse_listeners(&self) { unimplemented!() }
//
//    fn trigger_enter_rule_event(&self) { unimplemented!() }
//
//    fn trigger_exit_rule_event(&self) { unimplemented!() }
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
//
//    fn add_context_to_parse_tree(&self) { unimplemented!() }
//
    pub(crate) fn enter_rule(&mut self, localctx: &mut dyn RuleContext, state: isize, rule_index: usize) {
        self.set_state(state);
        if self.build_parse_trees {
            unimplemented!()
        }
    }
    //
    pub(crate) fn exit_rule(&mut self, mut localctx: &mut Option<Box<dyn RuleContext>>) {
        let mut ctx = localctx.as_mut().unwrap();
        self.set_state(ctx.get_invoking_state());
        let mut parent = ctx.get_parent_ctx().take();
        mem::replace(localctx, parent);
    }
    //
//
    pub(crate) fn enter_outer_alt(&mut self, localctx: &mut dyn RuleContext, altNum: isize) {}
//
//
//    fn enter_recursion_rule(&self, localctx: ParserRuleContext, state: isize, ruleIndex: isize, precedence: isize) { unimplemented!() }
//
//    fn push_new_recursion_context(&self, localctx: ParserRuleContext, state: isize, ruleIndex: isize) { unimplemented!() }
//
//    fn unroll_recursion_contexts(&self, parentCtx: ParserRuleContext) { unimplemented!() }
//
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

