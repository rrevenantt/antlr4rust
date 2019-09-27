use crate::recognizer::Recognizer;
use crate::errors::ANTLRError;
use crate::lexer::{Lexer, BaseLexer};
use crate::token::OwningToken;

pub trait ErrorListener {
    fn syntax_error(
        &mut self,
        recognizer: &BaseLexer,
        offending_symbol: Option<&OwningToken>,
        line: isize,
        column: isize,
        msg: &str,
        e: &ANTLRError,
    );
    //    fn report_ambiguity(recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, exact: bool, ambigAlts: * BitSet, configs: ATNConfigSet)
    //    fn report_attempting_full_context(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, conflictingAlts: * BitSet, configs: ATNConfigSet);
    //    fn report_context_sensitivity(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, prediction: isize, configs: ATNConfigSet);
}

#[derive(Debug)]
pub struct DefaultErrorListener {}

impl ErrorListener for DefaultErrorListener {
    fn syntax_error(&mut self, recognizer: &BaseLexer, offending_symbol: Option<&OwningToken>, line: isize, column: isize, msg: &str, e: &ANTLRError) {
        eprintln!("line {}:{} {}", line, column, msg);
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
