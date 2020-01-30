use std::any::Any;
use std::cell::Ref;
use std::ops::Deref;

use bit_set::BitSet;

use crate::atn_config_set::ATNConfigSet;
use crate::dfa::DFA;
use crate::errors::ANTLRError;
use crate::parser::Parser;
use crate::recognizer::Recognizer;
use crate::token::Token;

pub trait ErrorListener {
    fn syntax_error(&self, _recognizer: &dyn Any, _offending_symbol: Option<&dyn Token>,
                    _line: isize, _column: isize, _msg: &str, _e: Option<&ANTLRError>, ) {}

    fn report_ambiguity(&self, _recognizer: &dyn Parser, _dfa: &DFA, _start_index: isize, _stop_index: isize,
                        _exact: bool, _ambig_alts: &BitSet, _configs: &ATNConfigSet) {}

    fn report_attempting_full_context(&self, _recognizer: &dyn Parser, _dfa: &DFA, _start_index: isize, _stop_index: isize,
                                      _conflicting_alts: &BitSet, _configs: &ATNConfigSet) {}

    fn report_context_sensitivity(&self, _recognizer: &dyn Parser, _dfa: &DFA, _start_index: isize,
                                  _stop_index: isize, _prediction: isize, _configs: &ATNConfigSet) {}
}

#[derive(Debug)]
pub struct ConsoleErrorListener {}

impl ErrorListener for ConsoleErrorListener {
    fn syntax_error(&self, _recognizer: &dyn Any, _offending_symbol: Option<&dyn Token>,
                    line: isize, column: isize, msg: &str, _e: Option<&ANTLRError>) {
        eprintln!("line {}:{} {}", line, column, msg);
    }
}

pub struct ProxyErrorListener<'a> {
    pub delegates: Ref<'a, Vec<Box<dyn ErrorListener>>>
}

impl<'a> ErrorListener for ProxyErrorListener<'a> {
    fn syntax_error(&self, recognizer: &dyn Any, offending_symbol: Option<&dyn Token>, line: isize, column: isize, msg: &str, e: Option<&ANTLRError>) {
        for listener in self.delegates.deref() {
            listener.syntax_error(recognizer, offending_symbol, line, column, msg, e)
        }
    }

    fn report_ambiguity(&self, recognizer: &dyn Parser, dfa: &DFA, start_index: isize, stop_index: isize, exact: bool, ambig_alts: &BitSet<u32>, configs: &ATNConfigSet) {
        for listener in self.delegates.deref() {
            listener.report_ambiguity(recognizer, dfa, start_index, stop_index, exact, ambig_alts, configs)
        }
    }

    fn report_attempting_full_context(&self, recognizer: &dyn Parser, dfa: &DFA, start_index: isize, stop_index: isize,
                                      conflicting_alts: &BitSet<u32>, configs: &ATNConfigSet) {
        for listener in self.delegates.deref() {
            listener.report_attempting_full_context(recognizer, dfa, start_index, stop_index,
                                                    conflicting_alts, configs)
        }
    }

    fn report_context_sensitivity(&self, recognizer: &dyn Parser, dfa: &DFA, start_index: isize, stop_index: isize, prediction: isize, configs: &ATNConfigSet) {
        for listener in self.delegates.deref() {
            listener.report_context_sensitivity(recognizer, dfa, start_index, stop_index, prediction, configs)
        }
    }
}

pub struct DiagnosticErrorListener {
    exact_only: bool
}

impl DiagnosticErrorListener {
    pub fn new(exact_only: bool) -> Self { Self { exact_only } }

    fn get_decision_description(&self, recog: &dyn Parser, dfa: &DFA) -> String {
        let decision = dfa.decision;
        let rule_index = recog.get_atn().states[dfa.atn_start_state].get_rule_index();

        let rule_names = recog.get_rule_names();
        if let Some(&rule_name) = rule_names.get(rule_index as usize) {
            format!("{} ({})", decision, rule_name)
        } else {
            decision.to_string()
        }
    }

    fn get_conflicting_alts<'a>(&self, alts: &'a BitSet, _configs: &ATNConfigSet) -> &'a BitSet {
        //alts is never None
        alts
    }
}

impl ErrorListener for DiagnosticErrorListener {
    fn report_ambiguity(&self, recognizer: &dyn Parser, dfa: &DFA, start_index: isize, stop_index: isize, exact: bool, ambig_alts: &BitSet<u32>, _configs: &ATNConfigSet) {
        if self.exact_only && !exact { return }
        let msg = format!("reportAmbiguity d={}: ambigAlts={:?}, input='{}'",
                          self.get_decision_description(recognizer, dfa),
                          ambig_alts,
                          recognizer.get_input_stream().get_text_from_interval(start_index, stop_index)
        );
        recognizer.notify_error_listeners(msg, None, None);
    }

    fn report_attempting_full_context(&self, recognizer: &dyn Parser, dfa: &DFA, start_index: isize, stop_index: isize,
                                      _conflicting_alts: &BitSet<u32>, _configs: &ATNConfigSet) {
        let msg = format!("reportAttemptingFullContext d={}, input='{}'",
                          self.get_decision_description(recognizer, dfa),
                          recognizer.get_input_stream().get_text_from_interval(start_index, stop_index)
        );
        recognizer.notify_error_listeners(msg, None, None);
    }

    fn report_context_sensitivity(&self, recognizer: &dyn Parser, dfa: &DFA, start_index: isize, stop_index: isize, _prediction: isize, _configs: &ATNConfigSet) {
        let msg = format!("reportContextSensitivity d={}, input='{}'",
                          self.get_decision_description(recognizer, dfa),
                          recognizer.get_input_stream().get_text_from_interval(start_index, stop_index)
        );
        recognizer.notify_error_listeners(msg, None, None);
    }
}
/*
impl DefaultErrorListener {
    fn new_default_error_listener() -> * DefaultErrorListener { unimplemented!() }

    fn syntax_error(&self, recognizer: Recognizer, offendingSymbol: interface {
    }, line: isize, column: isize, msg: String, e: RecognitionError) { unimplemented!() }

    fn report_ambiguity(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, exact: bool, ambigAlts: * BitSet, configs: ATNConfigSet) { unimplemented!() }

    fn report_attempting_full_context(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, conflictingAlts: * BitSet, configs: ATNConfigSet) { unimplemented!() }

    fn report_context_sensitivity(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, prediction: isize, configs: ATNConfigSet) { unimplemented!() }

    pub struct ConsoleErrorListener {
    base: DefaultErrorListener,
    }

    fn new_console_error_listener() -> * ConsoleErrorListener { unimplemented!() }

    var ConsoleErrorListenerINSTANCE = NewConsoleErrorListener()

    fn syntax_error(&self, recognizer: Recognizer, offendingSymbol: interface {
    }, line: isize, column: isize, msg: String, e: RecognitionError) {
        fmt.Fprintln(os.Stderr, "line " + strconv.Itoa(line) + ":" + strconv.Itoa(column) + " " + msg)
    }

    pub struct ProxyErrorListener {
    base: DefaultErrorListener,
    delegates: Vec < ErrorListener > ,
    }

    fn new_proxy_error_listener(delegates Vec<ErrorListener>) -> * ProxyErrorListener { unimplemented!() }

    fn syntax_error(&self, recognizer: Recognizer, offendingSymbol: interface {
    }, line: isize, column: isize, msg: String, e: RecognitionError) {
        for _, d: = range p.delegates {
            d.SyntaxError(recognizer, offendingSymbol, line, column, msg, e)
        }
    }

    fn report_ambiguity(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, exact: bool, ambigAlts: * BitSet, configs: ATNConfigSet) { unimplemented!() }

    fn report_attempting_full_context(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, conflictingAlts: * BitSet, configs: ATNConfigSet) { unimplemented!() }

    fn report_context_sensitivity(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, prediction: isize, configs: ATNConfigSet) { unimplemented!() }
}
 */
