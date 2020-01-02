use std::iter::Map;
use std::sync::Arc;

use crate::atn::ATN;
use crate::char_stream::CharStream;
use crate::error_listener::ErrorListener;
use crate::errors::ANTLRError;
use crate::lexer::{BaseLexer, Lexer};
use crate::parser_rule_context::ParserRuleContext;
use crate::rule_context::{BaseRuleContext, RuleContext};
use crate::token::Token;
use crate::vocabulary::Vocabulary;

pub trait Recognizer {
    //    fn get_literal_names(&self) -> &[Option<&str>] {
//        &[]
//    }
//    fn get_symbolic_names(&self) -> &[Option<&str>] {
//        &[]
//    }
//    fn sempred(&mut self, _localctx: Option<&dyn ParserRuleContext>, rule_index: isize, action_index: isize) -> bool { true }
//    fn action(&mut self, _localctx: Option<&dyn ParserRuleContext>, rule_index: isize, action_index: isize) {}

    fn get_rule_names(&self) -> &[&str] {
        &[]
    }
    fn get_vocabulary(&self) -> &dyn Vocabulary { unimplemented!() }

    fn get_grammar_file_name(&self) -> &str { "" }
    fn get_atn(&self) -> &ATN { unimplemented!() }
}

pub trait Actions {
    type Recog: ?Sized;
    //todo make self and recog mutable,
    fn sempred(&self, _localctx: Option<&dyn ParserRuleContext>, rule_index: isize, action_index: isize,
               recog: &Self::Recog,
    ) -> bool {
        true
    }

    fn action(&mut self, _localctx: Option<&dyn ParserRuleContext>, rule_index: isize, action_index: isize,
              recog: &mut Self::Recog,
    ) {}
}

//impl Recognizer for BaseRecognizer {
//    fn get_state(&self) -> isize {
//        self.state
//    }
//
//    fn set_state(&mut self, _v: isize) {
//        self.state = _v;
//    }
//
//    fn add_error_listener(&mut self, _listener: Box<ErrorListener>) {
//        self.listeners.push(_listener)
//    }
//
//    fn remove_error_listeners(&self) {
//        unimplemented!()
//    }
//
//    fn get_error_listener_dispatch(&self) -> Box<ErrorListener> {
//        unimplemented!()
//    }
//}
//
//pub struct BaseRecognizer {
//    pub listeners: Vec<Box<ErrorListener>>,
//    pub state: isize, //    rule_names: Vec<String>,
//    //    literal_names: Vec<String>,
//    //    symbolic_names: Vec<String>,
//    //    grammar_file_name: String
//}
//
//impl BaseRecognizer {
//    pub fn new_base_recognizer() -> BaseRecognizer {
//        BaseRecognizer {
//            listeners: Vec::new(),
//            state: -1,
//        }
//    }
//
//    fn check_version(&self, _toolVersion: String) {
//        unimplemented!()
//    }
//
//    fn get_token_names(&self) -> Vec<String> {
//        unimplemented!()
//    }
//
//    fn get_rule_index_map(&self) -> Map<isize, String> {
//        unimplemented!()
//    }
//
//    fn get_token_type(&self, _tokenName: String) -> isize {
//        unimplemented!()
//    }
//
//    fn get_error_header(&self, _e: ANTLRError) -> String {
//        unimplemented!()
//    }
//
//    fn get_token_error_display(&self, _t: &Token) -> String {
//        unimplemented!()
//    }
//}
