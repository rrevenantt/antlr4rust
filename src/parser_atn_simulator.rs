//var (
//parser_atnsimulator_debug            = false
//parser_atnsimulator_list_atndecisions = false
//parser_atnsimulator_dfadebug         = false
//parser_atnsimulator_retry_debug       = false
//)


use crate::atn_simulator::{BaseATNSimulator, IATNSimulator};
use std::sync::Arc;
use crate::prediction_context::PredictionContextCache;
use crate::atn::ATN;
use crate::dfa::DFA;
use crate::rule_context::RuleContext;
use crate::recognizer::Recognizer;

pub struct ParserATNSimulator {
    base: BaseATNSimulator,

    //    parser: Parser,
    prediction_mode: isize,
    //    input: TokenStream,
    start_index: isize,
    //    dfa: * DFA,
//    merge_cache: * DoubleDict,
    outer_context: Option<Box<dyn RuleContext>>,
}

impl ParserATNSimulator {
    pub fn new(atn: Arc<ATN>, decision_to_dfa: Arc<Vec<DFA>>, shared_context_cache: Arc<PredictionContextCache>) -> ParserATNSimulator {
        ParserATNSimulator {
            base: BaseATNSimulator::new_base_atnsimulator(atn, decision_to_dfa, shared_context_cache),
            prediction_mode: 0,
            start_index: 0,
//            merge_cache: (),
            outer_context: None,
        }
    }

    fn get_prediction_mode(&self) -> isize { unimplemented!() }

    fn set_prediction_mode(&self, v: isize) { unimplemented!() }

    fn reset(&self) { unimplemented!() }

//    fn adaptive_predict(&self, input: TokenStream, decision: isize, outerContext: ParserRuleContext) -> int { unimplemented!() }

//
//    fn exec_atn(&self, dfa: * DFA, s0: * DFAState, input: TokenStream, startIndex: isize, outerContext: ParserRuleContext) -> int { unimplemented!() }
//
//
//    fn get_existing_target_state(&self, previousD: * DFAState, t: isize) -> * DFAState { unimplemented!() }
//
//
//    fn compute_target_state(&self, dfa: * DFA, previousD: * DFAState, t: isize) -> * DFAState { unimplemented!() }
//
//    fn predicate_dfastate(&self, dfaState: * DFAState, decisionState: DecisionState) { unimplemented!() }
//
//    fn exec_atn_with_full_context(&self, dfa: * DFA, D: * DFAState, s0: ATNConfigSet, input: TokenStream, startIndex: isize, outerContext: ParserRuleContext) -> int { unimplemented!() }
//
//    fn compute_reach_set(&self, closure: ATNConfigSet, t: isize, fullCtx: bool) -> ATNConfigSet { unimplemented!() }
//
//    fn remove_all_configs_not_in_rule_stop_state(&self, configs: ATNConfigSet, lookToEndOfRule: bool) -> ATNConfigSet { unimplemented!() }
//
//    fn compute_start_state(&self, a: ATNState, ctx: RuleContext, fullCtx: bool) -> ATNConfigSet { unimplemented!() }
//
//    fn apply_precedence_filter(&self, configs: ATNConfigSet) -> ATNConfigSet { unimplemented!() }
//
//    fn get_reachable_target(&self, trans: Transition, ttype: isize) -> ATNState { unimplemented!() }
//
//    fn get_preds_for_ambig_alts(&self, ambigAlts: * BitSet, configs: ATNConfigSet, nalts: isize) -> Vec<SemanticContext> { unimplemented!() }
//
//    fn get_predicate_predictions(&self, ambigAlts: * BitSet, altToPred Vec<SemanticContext>) -> &Vec<PredPrediction> { unimplemented!() }
//
//    fn get_syn_valid_or_sem_invalid_alt_that_finished_decision_entry_rule(&self, configs: ATNConfigSet, outerContext: ParserRuleContext) -> int { unimplemented!() }
//
//    fn get_alt_that_finished_decision_entry_rule(&self, configs: ATNConfigSet) -> int { unimplemented!() }
}

impl IATNSimulator for ParserATNSimulator {
    fn shared_context_cache(&self) -> Arc<PredictionContextCache> {
        self.base.shared_context_cache()
    }

    fn atn(&self) -> &ATN {
        self.base.atn()
    }

    fn decision_to_dfa(&self) -> &Vec<DFA> {
        self.base.decision_to_dfa()
    }
}
//    pub struct ATNConfigSetPair {
//    item0: ATNConfigSet,
//    item1: ATNConfigSet
//    }
//impl ATNConfigSetPair{
//    fn split_according_to_semantic_validity(&self, configs: ATNConfigSet, outerContext: ParserRuleContext) -> Vec<ATNConfigSet> { unimplemented!() }
//
//    fn eval_semantic_context(&self, predPredictions: &Vec<PredPrediction>, outerContext: ParserRuleContext, complete: bool) -> * BitSet { unimplemented!() }
//
//    fn closure(&self, config: ATNConfig, configs: ATNConfigSet, closureBusy: * Set, collectPredicates: bool, fullCtx: bool, treatEOFAsEpsilon: bool) { unimplemented!() }
//
//    fn closure_checking_stop_state(&self, config: ATNConfig, configs: ATNConfigSet, closureBusy: * Set, collectPredicates: bool, fullCtx: bool, depth: isize, treatEOFAsEpsilon: bool) { unimplemented!() }
//
//    fn closure_work(&self, config: ATNConfig, configs: ATNConfigSet, closureBusy: * Set, collectPredicates: bool, fullCtx: bool, depth: isize, treatEOFAsEpsilon: bool) { unimplemented!() }
//
//    fn get_rule_name(&self, index: isize) -> String { unimplemented!() }
//
//    fn get_epsilon_target(&self, config: ATNConfig, t: Transition, collectPredicates: bool, inContext: bool, fullCtx: bool, treatEOFAsEpsilon: bool) -> ATNConfig { unimplemented!() }
//
//    fn action_transition(&self, config: ATNConfig, t: * ActionTransition) -> * BaseATNConfig { unimplemented!() }
//
//    fn precedence_transition(&self, config: ATNConfig,
//                             pt: *PrecedencePredicateTransition, collectPredicates: bool, inContext: bool, fullCtx: bool) -> * BaseATNConfig { unimplemented!() }
//
//    fn pred_transition(&self, config: ATNConfig, pt: * PredicateTransition, collectPredicates: bool, inContext: bool, fullCtx: bool) -> * BaseATNConfig { unimplemented!() }
//
//    fn rule_transition(&self, config: ATNConfig, t: * RuleTransition) -> * BaseATNConfig { unimplemented!() }
//
//    fn get_conflicting_alts(&self, configs: ATNConfigSet) -> * BitSet { unimplemented!() }
//
//
//    fn get_conflicting_alts_or_unique_alt(&self, configs: ATNConfigSet) -> * BitSet { unimplemented!() }
//
//    fn get_token_name(&self, t: isize) -> String { unimplemented!() }
//
//    fn get_lookahead_name(&self, input: TokenStream) -> String { unimplemented!() }
//
//    fn dump_dead_end_configs(&self, nvae: * NoViableAltError) { unimplemented!() }
//
//    fn no_viable_alt(&self, input: TokenStream, outerContext: ParserRuleContext, configs: ATNConfigSet, startIndex: isize) -> * NoViableAltError { unimplemented!() }
//
//    fn get_unique_alt(&self, configs: ATNConfigSet) -> int { unimplemented!() }
//
//    fn add_dfaedge(&self, dfa: * DFA, from: * DFAState, t: isize, to: * DFAState) -> * DFAState { unimplemented!() }
//
//    fn add_dfastate(&self, dfa: * DFA, d: * DFAState) -> * DFAState { unimplemented!() }
//
//    fn report_attempting_full_context(&self, dfa: * DFA, conflictingAlts: * BitSet, configs: ATNConfigSet, startIndex: isize, stopIndex: isize) { unimplemented!() }
//
//    fn report_context_sensitivity(&self, dfa: * DFA, prediction: isize, configs: ATNConfigSet, startIndex: isize, stopIndex: isize) { unimplemented!() }
//
//    fn report_ambiguity(&self, dfa: * DFA, D: * DFAState, startIndex: isize, stopIndex: isize,
//                        exact: bool, ambigAlts: * BitSet, configs: ATNConfigSet) { unimplemented!() }
//}
//