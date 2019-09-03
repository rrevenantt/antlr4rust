pub struct LL1Analyzer {
    atn: * ATN,
}

impl LL1Analyzer {
    fn new_ll1_analyzer(atn: * ATN) -> * LL1Analyzer { unimplemented!() }

    fn get_decision_lookahead(&self, s: ATNState) -> &Vec<IntervalSet> { unimplemented!() }

    fn look(&self, s: ATNState, stopState: ATNState, ctx: RuleContext) -> * IntervalSet { unimplemented!() }


    fn look2(&self, s: ATNState, stopState: ATNState, ctx: PredictionContext, look: * IntervalSet, lookBusy: * Set, calledRuleStack: * BitSet, seeThruPreds: bool, addEOF: bool, i: isize) { unimplemented!() }

    fn look1(&self, s: ATNState, stopState: ATNState, ctx: PredictionContext, look: * IntervalSet, lookBusy: * Set, calledRuleStack: * BitSet, seeThruPreds: bool, addEOF: bool) { unimplemented!() }

    fn look3(&self, stopState: ATNState, ctx: PredictionContext, look: * IntervalSet, lookBusy: * Set, calledRuleStack: * BitSet, seeThruPreds: bool, addEOF: bool, t1: * RuleTransition) { unimplemented!() }
}
 