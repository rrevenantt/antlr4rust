pub struct DiagnosticErrorListener {
    base: DefaultErrorListener,

    exact_only: bool,
}

impl DiagnosticErrorListener {
    fn new_diagnostic_error_listener(exactOnly bool) -> * DiagnosticErrorListener { unimplemented!() }

    fn report_ambiguity(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, exact: bool, ambigAlts: * BitSet, configs: ATNConfigSet) { unimplemented!() }

    fn report_attempting_full_context(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, conflictingAlts: * BitSet, configs: ATNConfigSet) { unimplemented!() }

    fn report_context_sensitivity(&self, recognizer: Parser, dfa: * DFA, startIndex: isize, stopIndex: isize, prediction: isize, configs: ATNConfigSet) { unimplemented!() }

    fn get_decision_description(&self, recognizer: Parser, dfa: * DFA) -> String { unimplemented!() }

    fn get_conflicting_alts(&self, ReportedAlts: * BitSet, set: ATNConfigSet) -> * BitSet { unimplemented!() }
}
 