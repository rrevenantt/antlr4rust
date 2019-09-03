use crate::rule_context::RuleContext;
use crate::error_listener::ErrorListener;
use crate::atn::ATN;
use crate::errors::ANTLRError;
use crate::token::Token;
use std::iter::Map;

use std::sync::Arc;

pub trait Recognizer {
    fn get_literal_names(&self) -> &[Option<&str>] {
        &[]
    }
    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &[]
    }
    fn get_rule_names(&self) -> &[&str] {
        &[]
    }

    fn sempred(&self, _localctx: &RuleContext, _ruleIndex: isize, _actionIndex: isize) -> bool {
        true
    }
    fn precpred(&self, _localctx: &RuleContext, _precedence: isize) -> bool {
        true
    }
    fn action(&self, _context: &RuleContext, _ruleIndex: isize, _actionIndex: isize) {}

    fn get_atn(&self) -> Arc<ATN> {
        unreachable!()
    }
    fn get_grammar_file_name(&self) -> &str {
        ""
    }

    fn get_state(&self) -> isize;

    fn set_state(&mut self, v: isize);

    fn add_error_listener(&mut self, listener: Box<ErrorListener>);

    fn remove_error_listeners(&self);

    fn get_error_listener_dispatch(&self) -> Box<ErrorListener>;
}

impl Recognizer for BaseRecognizer {
    fn get_state(&self) -> isize {
        self.state
    }

    fn set_state(&mut self, _v: isize) {
        self.state = _v;
    }

    fn add_error_listener(&mut self, _listener: Box<ErrorListener>) {
        self.listeners.push(_listener)
    }

    fn remove_error_listeners(&self) {
        unimplemented!()
    }

    fn get_error_listener_dispatch(&self) -> Box<ErrorListener> {
        unimplemented!()
    }
}

pub struct BaseRecognizer {
    pub listeners: Vec<Box<ErrorListener>>,
    pub state: isize, //    rule_names: Vec<String>,
    //    literal_names: Vec<String>,
    //    symbolic_names: Vec<String>,
    //    grammar_file_name: String
}

impl BaseRecognizer {
    pub fn new_base_recognizer() -> BaseRecognizer {
        BaseRecognizer {
            listeners: Vec::new(),
            state: -1,
        }
    }

    fn check_version(&self, _toolVersion: String) {
        unimplemented!()
    }

    fn get_token_names(&self) -> Vec<String> {
        unimplemented!()
    }

    fn get_rule_index_map(&self) -> Map<isize, String> {
        unimplemented!()
    }

    fn get_token_type(&self, _tokenName: String) -> isize {
        unimplemented!()
    }

    fn get_error_header(&self, _e: ANTLRError) -> String {
        unimplemented!()
    }

    fn get_token_error_display(&self, _t: &Token) -> String {
        unimplemented!()
    }
}
