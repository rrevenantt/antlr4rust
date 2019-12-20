//var (
//#![allow(non_snake_case)]
//parser_atnsimulator_debug            = false
//parser_atnsimulator_list_atndecisions = false
//parser_atnsimulator_dfadebug         = false
//parser_atnsimulator_retry_debug       = false
//)


use std::borrow::BorrowMut;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;
use std::usize;

use bit_set::BitSet;
use murmur3::murmur3_32::MurmurHasher;

use crate::atn::{ATN, INVALID_ALT};
use crate::atn_config::ATNConfig;
use crate::atn_config_set::ATNConfigSet;
use crate::atn_deserializer::cast;
use crate::atn_simulator::{BaseATNSimulator, IATNSimulator};
use crate::atn_state::{ATNState, ATNStateRef};
use crate::atn_state::ATNStateType::{DecisionState, RuleStopState};
use crate::dfa::{DFA, ScopeExt};
use crate::dfa_state::{DFAState, DFAStateRef, PredPrediction};
use crate::errors::ANTLRError;
use crate::int_stream::EOF;
use crate::interval_set::IntervalSet;
use crate::lexer_atn_simulator::ERROR_DFA_STATE_REF;
use crate::parser::{BaseParser, Parser};
use crate::parser_rule_context::{BaseParserRuleContext, empty_ctx, ParserRuleContext};
use crate::prediction_context::{MurmurHasherBuilder, PREDICTION_CONTEXT_EMPTY_RETURN_STATE, PredictionContext, PredictionContextCache};
use crate::prediction_mode::{get_conflicting_alt_subsets, PredictionMode, resolves_to_just_one_viable_alt};
use crate::recognizer::Recognizer;
use crate::rule_context::RuleContext;
use crate::semantic_context::SemanticContext;
use crate::token::{TOKEN_EOF, TOKEN_EPSILON};
use crate::token_stream::TokenStream;
use crate::transition::{ActionTransition, EpsilonTransition, PrecedencePredicateTransition, PredicateTransition, RuleTransition, Transition, TRANSITION_ACTION, TransitionType};

pub struct ParserATNSimulator {
    base: BaseATNSimulator,

    //    parser: Parser,
    prediction_mode: PredictionMode,
    //    input: TokenStream,
    start_index: Cell<isize>,
    //    dfa: * DFA,
//    merge_cache: * DoubleDict,
//    outer_context: Option<Box<dyn RuleContext>>,
}

struct Local<'a, 'b> {
    //    input:&'a mut dyn TokenStream,
//    start_index:isize,
//    outer_context: &'a dyn ParserRuleContext,
    dfa: &'a DFA,
    merge_cache: &'b mut HashMap<(u64, u64), PredictionContext, MurmurHasherBuilder>,
    precedence: isize,
    parser: &'a mut dyn Parser,
}

impl Local<'_, '_> {
    fn input(&mut self) -> &mut dyn TokenStream { self.parser.get_input_stream() }
    fn seek(&mut self, i: isize) { self.input().seek(i) }
    fn outer_context(&self) -> &dyn ParserRuleContext { self.parser.get_parser_rule_context() }
}

pub type MergeCache = HashMap<(u64, u64), PredictionContext, MurmurHasherBuilder>;

impl ParserATNSimulator {
    pub fn new(atn: Arc<ATN>, decision_to_dfa: Arc<Vec<DFA>>, shared_context_cache: Arc<PredictionContextCache>) -> ParserATNSimulator {
        ParserATNSimulator {
            base: BaseATNSimulator::new_base_atnsimulator(atn, decision_to_dfa, shared_context_cache),
            prediction_mode: PredictionMode::LL,
//            merge_cache: (),
            start_index: Cell::new(0)
        }
    }

    fn get_prediction_mode(&self) -> PredictionMode { self.prediction_mode }

    fn set_prediction_mode(&mut self, v: PredictionMode) { self.prediction_mode = v }

    fn reset(&self) { unimplemented!() }

    pub fn adaptive_predict(&self,
//                        input: &mut dyn TokenStream,
                            decision: isize,
//                        outer_context: &dyn ParserRuleContext,
                            parser: &mut dyn Parser,
    ) -> Result<isize, ANTLRError> {
        self.start_index.set(parser.get_input_stream().index());
        let mut merge_cache: MergeCache = HashMap::with_hasher(MurmurHasherBuilder {});
        let mut local = Local {
//            input,
//            start_index,
//            outer_context,
            dfa: &self.decision_to_dfa()[decision as usize],
            merge_cache: &mut merge_cache,
            precedence: parser.get_precedence(),
            parser,
        };

        let m = local.input().mark();

        let result = {
            let s0 = if local.dfa.is_precedence_dfa() {
                local.dfa.get_precedence_start_state(local.precedence/*parser.get_precedence()*/)
            } else {
                local.dfa.s0.read().unwrap().as_ref().copied()
            };

            let s0 = s0.unwrap_or_else(|| {
                let mut s0_closure = self.compute_start_state(
                    local.dfa.atn_start_state,
                    PredictionContext::from_rule_context(self.atn(), empty_ctx().as_ref()),
                    false,
                    &mut local,
                );
                if local.dfa.is_precedence_dfa() {
                    let mut s0 = local.dfa.s0.read().unwrap().unwrap();
                    let s0_closure_updated = self.apply_precedence_filter(&s0_closure);
                    local.dfa.states.write().unwrap()[s0].configs = Box::new(s0_closure);

                    s0 = self.add_dfastate(&local.dfa, DFAState::new_dfastate(0, Box::new(s0_closure_updated)));

                    local.dfa.set_precedence_start_state(local.precedence, s0);
                    s0
                } else {
                    let mut s0 = self.add_dfastate(&local.dfa, DFAState::new_dfastate(0, Box::new(s0_closure)));
                    local.dfa.s0.write().unwrap().replace(s0);
                    s0
                }
            });


            self.exec_atn(&mut local, s0)?
        };

        local.input().seek(self.start_index.get());
        local.input().release(m);

        Ok(result)
    }


    fn exec_atn(&self, local: &mut Local, s0: DFAStateRef) -> Result<isize, ANTLRError> {
        let mut previousD = s0;

        let mut token = local.input().la(1);
        loop {
            let D = self.get_existing_target_state(local.dfa, previousD, token)
                .unwrap_or(self.compute_target_state(local.dfa, previousD, token, local));

            let states = local.dfa.states.read().unwrap();
            if D == ERROR_DFA_STATE_REF {
                let previousDstate = &states[previousD];
                local.input().seek(self.start_index.get());
                let alt = self.get_syn_valid_or_sem_invalid_alt_that_finished_decision_entry_rule(
                    previousDstate.configs.as_ref(),
                    local,
                );
                if alt != INVALID_ALT {
                    return Ok(alt);
                }
                return Err(self.no_viable_alt(local, previousDstate.configs.as_ref(), self.start_index.get()))
            }
//            let states = local.dfa.states.read().unwrap();
            let Dstate = &states[D];

            if Dstate.requires_full_context && self.prediction_mode == PredictionMode::SLL {
                let mut conflicting_alts = Dstate.configs.get_conflicting_alts().clone();//todo get rid of clone
                if !Dstate.predicates.is_empty() {
                    let conflict_index = local.input().index();
                    if conflict_index != self.start_index.get() {
                        local.input().seek(self.start_index.get())
                    }

                    conflicting_alts = self.eval_semantic_context(local, &Dstate.predicates, true);
                    if conflicting_alts.len() == 1 {
                        return Ok(conflicting_alts.iter().next().unwrap() as isize);
                    }

                    if conflict_index != self.start_index.get() {
                        local.input().seek(conflict_index)
                    }
                }

                let s0_closure = self.compute_start_state(
                    local.dfa.atn_start_state,
                    PredictionContext::from_rule_context(self.atn(), local.outer_context()),
                    true,
                    local,
                );

                self.report_attempting_full_context(
                    local.dfa,
                    &conflicting_alts,
                    Dstate.configs.as_ref(),
                    self.start_index.get(),
                    local.input().index(),
                );

                return self.exec_atn_with_full_context(local, &Dstate, s0_closure);
            }

            if Dstate.is_accept_state {
                if Dstate.predicates.is_empty() {
                    return Ok(Dstate.prediction)
                }

                let stop_index = local.input().index();
                local.input().seek(self.start_index.get());

                let alts = self.eval_semantic_context(local, &Dstate.predicates, true);
                match alts.len() {
                    0 => return Err(self.no_viable_alt(local, Dstate.configs.as_ref(), self.start_index.get())),
                    1 => return Ok(alts.iter().next().unwrap() as isize),
                    _ => {
                        self.report_ambiguity(
                            local.dfa,
                            self.start_index.get(),
                            stop_index,
                            false,
                            &alts,
                            Dstate.configs.as_ref(),
                        );
                        return Ok(alts.iter().next().unwrap() as isize)
                    }
                }
            }
            previousD = D;

            if token != EOF {
                local.input().consume();
                token = local.input().la(1);
            }
        }
    }

    fn get_existing_target_state(&self, dfa: &DFA, previousD: DFAStateRef, t: isize) -> Option<DFAStateRef> {
        dfa.states.read().unwrap()[previousD]
            .edges
            .get((t + 1) as usize)
            .and_then(|x| match *x {
                0 => None,
                x => Some(x)
            })
    }

    fn compute_target_state(&self, dfa: &DFA, previousD: DFAStateRef, t: isize, local: &mut Local) -> DFAStateRef {
        let reach = self.compute_reach_set(
            dfa.states.read().unwrap()[previousD].configs.as_ref(),
            t,
            false,
            local,
        );
        let reach = match reach {
            None => {
                self.add_dfaedge(dfa.states.write().unwrap()[previousD].borrow_mut(), t, ERROR_DFA_STATE_REF);
                return ERROR_DFA_STATE_REF
            },
            Some(x) => x,
        };

        let predicted_alt = self.get_unique_alt(&reach);

        let mut D = DFAState::new_dfastate(0, reach.into());

        if predicted_alt != INVALID_ALT {
            D.is_accept_state = true;
            D.configs.set_unique_alt(predicted_alt);
            D.prediction = predicted_alt
        } else if false /*todo*/ {
            let alts = self.get_conflicting_alts(D.configs.as_ref());
            D.prediction = alts.iter().next().unwrap() as isize;
            D.configs.set_conflicting_alts(alts);
            D.requires_full_context = true;
            D.is_accept_state = true;
        }

        if D.is_accept_state && D.configs.has_semantic_context() {
            //todo
            unimplemented!()
        }


        let D = self.add_dfastate(dfa, D);
        self.add_dfaedge(dfa.states.write().unwrap()[previousD].borrow_mut(), t, D);
        D
    }

//    fn predicate_dfastate(&self, dfaState: * DFAState, decisionState: DecisionState) { unimplemented!() }

    fn exec_atn_with_full_context(&self, local: &mut Local, D: &DFAState, s0: ATNConfigSet) -> Result<isize, ANTLRError> {
        let full_ctx = true;
        let found_exact_ambig = false;
        let mut prev = s0;
        local.input().seek(self.start_index.get());
        let mut t = local.input().la(1);
        let mut predicted_alt = 0;
        loop {
            let reach = self.compute_reach_set(&prev, t, full_ctx, local);
            prev = match reach {
                None => {
                    local.input().seek(self.start_index.get());
                    let alt = self.get_syn_valid_or_sem_invalid_alt_that_finished_decision_entry_rule(&prev, local);
                    if alt != INVALID_ALT {
                        return Ok(alt)
                    }
                    return Err(self.no_viable_alt(local, &prev, self.start_index.get()))
                },
                Some(x) => x,
            };

//            prev = reach;

            let alt_sub_sets = get_conflicting_alt_subsets(&prev);
            prev.set_unique_alt(self.get_unique_alt(&prev));
            if prev.get_unique_alt() != INVALID_ALT {
                predicted_alt = prev.get_unique_alt();
                break
            }
            if self.prediction_mode != PredictionMode::LL_EXACT_AMBIG_DETECTION {
                predicted_alt = resolves_to_just_one_viable_alt(&alt_sub_sets);
                if predicted_alt != INVALID_ALT {
                    break
                }
            } else {
                unimplemented!()
            }

            if t != TOKEN_EOF {
                local.input().consume();
                t = local.input().la(1);
            }
        }

        if prev.get_unique_alt() != INVALID_ALT {
            self.report_context_sensitivity(local.dfa, predicted_alt, &prev, self.start_index.get(), local.input().index())
        }

        self.report_ambiguity(local.dfa, self.start_index.get(), local.input().index(), found_exact_ambig, prev.get_conflicting_alts(), &prev);

        Ok(predicted_alt)
    }

    fn compute_reach_set(&self, closure: &ATNConfigSet, t: isize, full_ctx: bool, local: &mut Local) -> Option<ATNConfigSet> {
        let mut intermediate = ATNConfigSet::new_base_atnconfig_set(full_ctx);

        let mut skipped_stop_states = Vec::<&ATNConfig>::new();

        for c in closure.get_items() {
            let state = self.atn().states[c.get_state()].as_ref();
            if let RuleStopState = state.get_state_type() {
                assert!(c.get_context().unwrap().is_empty());
                if full_ctx || t == TOKEN_EOF {
                    skipped_stop_states.push(c);
                }
                continue
            }

            for tr in state.get_transitions() {
                self.get_reachable_target(tr.as_ref(), t)
                    .map(|target|
                        intermediate.add_cached(c.cloned(self.atn().states[target].as_ref()).into(), Some(local.merge_cache))
                    );
            }
        }

        let mut look_to_end_of_rule = false;
        let mut reach = if skipped_stop_states.is_empty() && t != TOKEN_EOF
            && (intermediate.length() == 1 || self.get_unique_alt(&intermediate) != INVALID_ALT) {
            look_to_end_of_rule = true;
            intermediate
        } else {
            let mut reach = ATNConfigSet::new_base_atnconfig_set(full_ctx);
            for c in intermediate.configs {
                let treat_eofas_epsilon = t == TOKEN_EOF;
                self.closure(*c, &mut reach, false, full_ctx, treat_eofas_epsilon, local);
            }
            reach
        };

        if t == TOKEN_EOF {
            reach = self.remove_all_configs_not_in_rule_stop_state(reach, look_to_end_of_rule, local.merge_cache);
        }

        if skipped_stop_states.is_empty() && (!full_ctx || !self.has_config_in_rule_stop_state(&reach)) {
            for c in skipped_stop_states {
                reach.add_cached(c.clone().into(), Some(local.merge_cache));
            }
        }

        if reach.is_empty() { return None }

        return Some(reach)
    }

    fn has_config_in_rule_stop_state(&self, configs: &ATNConfigSet) -> bool {
        for c in configs.get_items() {
            if let RuleStopState = self.atn().states[c.get_state()].get_state_type() {
                return true
            }
        }
        return false
    }

    fn all_configs_in_rule_stop_state(&self, configs: &ATNConfigSet) -> bool {
        for c in configs.get_items() {
            if let RuleStopState = self.atn().states[c.get_state()].get_state_type() {} else { return false }
        }
        return true
    }


    fn remove_all_configs_not_in_rule_stop_state(&self, configs: ATNConfigSet, look_to_end_of_rule: bool, merge_cache: &mut MergeCache) -> ATNConfigSet {
        if self.all_configs_in_rule_stop_state(&configs) {
            return configs
        }

        let mut result = ATNConfigSet::new_base_atnconfig_set(configs.full_context());
        for c in configs.configs {
            let state = self.atn().states[c.get_state()].as_ref();
            if let RuleStopState = state.get_state_type() {
                result.add_cached(c, Some(merge_cache));
                continue
            }

            if look_to_end_of_rule && state.has_epsilon_only_transitions() {
                let next_tokens = self.atn().next_tokens(state);
                if next_tokens.contains(TOKEN_EPSILON) {
                    let end_of_rule_state = self.atn().rule_to_stop_state[state.get_rule_index()];
                    result.add_cached(c.cloned(self.atn().states[end_of_rule_state].as_ref()).into(), Some(merge_cache));
                }
            }
        }

        result
    }

    fn compute_start_state(&self,
                           a: ATNStateRef,
                           initial_ctx: PredictionContext,
                           full_ctx: bool,
                           local: &mut Local,
    ) -> ATNConfigSet {
//        let initial_ctx = PredictionContext::prediction_context_from_rule_context(self.atn(),ctx);
        let mut configs = ATNConfigSet::new_base_atnconfig_set(full_ctx);

        let atn_states = &self.atn().states;
        for (i, tr) in atn_states[a].get_transitions().iter().enumerate() {
            let target = &atn_states[tr.get_target()];
            let c = ATNConfig::new(
                target.get_state_number(),
                (i + 1) as isize,
                Some(initial_ctx.clone()),
            );
            self.closure(c, &mut configs, true, full_ctx, false, local);
        }

        configs
    }

    fn apply_precedence_filter(&self, configs: &ATNConfigSet) -> ATNConfigSet {
        let config_set = ATNConfigSet::new_base_atnconfig_set(configs.full_context());

        for config in configs.get_items() {
            if config.get_alt() != 1 {
                continue
            }

//            if let Some(updated_sem_ctx) = config.semantic_context.as_deref().unwrap().eval_precedence(){
            unimplemented!();
//            }
        }

        config_set
    }

    fn get_reachable_target(&self, trans: &dyn Transition, ttype: isize) -> Option<ATNStateRef> {
        if trans.matches(ttype, 0, self.atn().max_token_type) {
            return Some(trans.get_target())
        }
        None
    }
//
//    fn get_preds_for_ambig_alts(&self, ambigAlts: * BitSet, configs: ATNConfigSet, nalts: isize) -> Vec<SemanticContext> { unimplemented!() }
//
//    fn get_predicate_predictions(&self, ambigAlts: * BitSet, altToPred Vec<SemanticContext>) -> &Vec<PredPrediction> { unimplemented!() }

    fn get_syn_valid_or_sem_invalid_alt_that_finished_decision_entry_rule(&self,
                                                                          configs: &ATNConfigSet,
                                                                          local: &Local,
    ) -> isize {
        let (sem_valid_configs, sem_invalid_configs) =
            self.split_according_to_semantic_validity(configs, local);

        let mut alt = self.get_alt_that_finished_decision_entry_rule(&sem_valid_configs);
        if alt != INVALID_ALT { return alt }

        if !sem_invalid_configs.is_empty() {
            let mut alt = self.get_alt_that_finished_decision_entry_rule(&sem_invalid_configs);
            if alt != INVALID_ALT { return alt }
        }

        INVALID_ALT
    }

    fn split_according_to_semantic_validity(&self, configs: &ATNConfigSet, local: &Local) -> (ATNConfigSet, ATNConfigSet) {
        let mut succeeded = ATNConfigSet::new_base_atnconfig_set(configs.full_context());
        let mut failed = ATNConfigSet::new_base_atnconfig_set(configs.full_context());
        for c in configs.get_items() {
            let clone = Box::new(c.clone());
            if c.get_semantic_context().map(|sem| sem != &SemanticContext::NONE) == Some(true) {
                let predicate_eval_result = c.get_semantic_context().unwrap().evaluate(local.parser);
                if predicate_eval_result {
                    succeeded.add(clone);
                } else {
                    failed.add(clone);
                }
            } else {
                succeeded.add(clone);
            }
        }
        (succeeded, failed)
    }

    fn get_alt_that_finished_decision_entry_rule(&self, configs: &ATNConfigSet) -> isize {
        let mut alts = IntervalSet::new();
        for c in configs.get_items() {
            let has_empty_path = c.get_context().map(|x| x.has_empty_path()) == Some(true);
            let is_stop = self.atn().states[c.get_state()].get_state_type() == &RuleStopState;
            if c.get_reaches_into_outer_context() > 0 || (is_stop && has_empty_path) {
                alts.add_one(c.get_alt())
            }
        }

        return alts.get_min().unwrap_or(INVALID_ALT);
    }

    fn eval_semantic_context(&self, local: &Local, pred_predictions: &Vec<PredPrediction>, complete: bool) -> BitSet {
        let mut predictions = BitSet::new();
        for pred in pred_predictions {
            if pred.pred == SemanticContext::NONE {
                predictions.insert(pred.alt as usize);

                if !complete { break }
                continue
            }

            let full_ctx = false;
            let predicate_evaluation_result = pred.pred.evaluate(local.parser);

            if predicate_evaluation_result {
                predictions.insert(pred.alt as usize);
                if !complete { break }
            }
        }
        predictions
    }

    fn closure(&self,
               config: ATNConfig,
               configs: &mut ATNConfigSet,
//               closure_busy: &mut HashSet<u64>,
               collect_predicates: bool,
               full_ctx: bool,
               treat_eofas_epsilon: bool,
               local: &mut Local,
    ) {
        let initial_depth = 0;

        //fixme hash collisions
        // maybe add list of possible duplicates
        let mut closure_busy: HashSet<u64> = HashSet::new();
        self.closure_checking_stop_state(
            config,
            configs,
            &mut closure_busy,
            collect_predicates,
            full_ctx,
            initial_depth,
            treat_eofas_epsilon,
            local,
        );
        assert!(!full_ctx || !configs.get_dips_into_outer_context())
    }

    fn closure_checking_stop_state(&self,
                                   mut config: ATNConfig,
                                   configs: &mut ATNConfigSet,
                                   closure_busy: &mut HashSet<u64>,
                                   collect_predicates: bool,
                                   full_ctx: bool,
                                   depth: isize,
                                   treat_eofas_epsilon: bool,
                                   local: &mut Local,
    ) {
        if let RuleStopState = self.atn().states[config.get_state()].get_state_type() {
            if !config.get_context().unwrap().is_empty() {
                let temp = config.get_context().unwrap().run(|temp| {
                    if temp.get_return_state(temp.length() - 1) == PREDICTION_CONTEXT_EMPTY_RETURN_STATE {
                        if full_ctx {
                            let new_config = config.cloned_with_new_ctx(
                                self.atn().states[config.get_state()].as_ref(),
                                Some(PredictionContext::new_empty()),
                            );
                            configs.add_cached(Box::new(new_config), Some(local.merge_cache));
                        } else {
                            self.closure_work(
                                config.clone(),
                                configs,
                                closure_busy,
                                collect_predicates,
                                full_ctx,
                                depth,
                                treat_eofas_epsilon,
                                local,
                            )
                        }
                    }
                });
                let mut context = config.take_context();
                for i in 0..context.length() {
                    if context.get_return_state(i) == PREDICTION_CONTEXT_EMPTY_RETURN_STATE {
                        if i != context.length() - 1 { panic!("EMPTY_RETURN_STATE is not last for some reason, please report error") }
                        continue
                    }
                    let return_state = context.get_return_state(i) as ATNStateRef;
                    let new_ctx = context.take_parent(i).unwrap();
                    let mut c = ATNConfig::new_with_semantic(
                        return_state,
                        config.get_alt(),
                        Some(new_ctx),
                        config.semantic_context.take(),
                    );
                    c.set_reaches_into_outer_context(config.get_reaches_into_outer_context());
                    assert!(depth > isize::min_value());
                    self.closure_checking_stop_state(
                        c,
                        configs,
                        closure_busy,
                        collect_predicates,
                        full_ctx,
                        depth,
                        treat_eofas_epsilon,
                        local,
                    )
                }
                return
            } else if full_ctx {
                configs.add_cached(Box::new(config), Some(local.merge_cache));
                return
            } else {}
        }
        self.closure_work(
            config,
            configs,
            closure_busy,
            collect_predicates,
            full_ctx,
            depth,
            treat_eofas_epsilon,
            local,
        )
    }

    fn closure_work(&self,
                    config: ATNConfig,
                    configs: &mut ATNConfigSet,
                    closure_busy: &mut HashSet<u64>,
                    collect_predicates: bool,
                    full_ctx: bool,
                    depth: isize,
                    treat_eofas_epsilon: bool,
                    local: &mut Local,
    ) {
        let p = self.atn().states[config.get_state()].as_ref();
        if !p.has_epsilon_only_transitions() {
            configs.add_cached(Box::new(config.clone()), Some(local.merge_cache));
        }

        for (i, tr) in p.get_transitions().iter().enumerate() {
            if i == 0 && self.can_drop_loop_entry_edge_in_left_recursive_rule(&config) { continue }

            let continue_collecting =
                tr.get_serialization_type() != TransitionType::TRANSITION_ACTION && collect_predicates;
            let c = self.get_epsilon_target(
                &config,
                tr.as_ref(),
                continue_collecting,
                depth == 0,
                full_ctx,
                treat_eofas_epsilon,
            );
            if let Some(mut c) = c {
                let mut new_depth = depth;
                let hash = c.default_hash();
                if let RuleStopState = self.atn().states[config.get_state()].get_state_type() {
                    assert!(!full_ctx);

                    if local.dfa.is_precedence_dfa() {
                        if tr.get_serialization_type() != TransitionType::TRANSITION_EPSILON { panic!("cast error"); }
                        let outermost_precedence_return =
                            unsafe { cast::<EpsilonTransition>(tr.as_ref()) }.outermost_precedence_return;
                        if outermost_precedence_return == local.dfa.atn_start_state as isize {
                            c.set_precedence_filter_suppressed(true);
                        }
                    }

                    c.reaches_into_outer_context += 1;
                    if !closure_busy.insert(hash) {
                        continue
                    }
//                    configs.set_dips_into_outer_context(true);
                    assert!(new_depth > isize::min_value());
                    new_depth -= 1;
                } else {
                    if !tr.is_epsilon() && !closure_busy.insert(hash) {
                        continue
                    }

                    if tr.get_serialization_type() == TransitionType::TRANSITION_RULE {
                        if new_depth >= 0 {
                            new_depth += 1
                        }
                    }
                }

                self.closure_checking_stop_state(
                    c,
                    configs,
                    closure_busy,
                    continue_collecting,
                    full_ctx,
                    new_depth,
                    treat_eofas_epsilon,
                    local,
                )
            };
        }
    }

    fn can_drop_loop_entry_edge_in_left_recursive_rule(&self, config: &ATNConfig) -> bool {
        //todo
        false
    }
//
//    fn get_rule_name(&self, index: isize) -> String { unimplemented!() }

    fn get_epsilon_target(&self,
                          config: &ATNConfig,
                          t: &dyn Transition,
                          collect_predicates: bool,
                          in_context: bool,
                          full_ctx: bool,
                          treat_eofas_epsilon: bool,
    ) -> Option<ATNConfig> {
        match t.get_serialization_type() {
            TransitionType::TRANSITION_EPSILON => Some(config.cloned(self.atn().states[t.get_target()].as_ref())),
            TransitionType::TRANSITION_RULE => Some(self.rule_transition(config, unsafe { cast::<RuleTransition>(t) })),
            TransitionType::TRANSITION_PREDICATE =>
                Some(self.pred_transition(
                    config,
                    unsafe { cast::<PredicateTransition>(t) },
                    collect_predicates,
                    in_context,
                    full_ctx,
                )),
            TransitionType::TRANSITION_ACTION => Some(self.action_transition(config, unsafe { cast::<ActionTransition>(t) })),
            TransitionType::TRANSITION_PRECEDENCE =>
                Some(self.precedence_transition(
                    config,
                    unsafe { cast::<PrecedencePredicateTransition>(t) },
                    collect_predicates,
                    in_context,
                    full_ctx,
                )),
            TransitionType::TRANSITION_ATOM |
            TransitionType::TRANSITION_SET |
            TransitionType::TRANSITION_RANGE =>
                if treat_eofas_epsilon && t.matches(TOKEN_EOF, 0, 1) {
                    Some(config.cloned(self.atn().states[t.get_target()].as_ref()))
                } else {
                    None
                },
            TransitionType::TRANSITION_NOTSET |
            TransitionType::TRANSITION_WILDCARD => None,
        }
    }

    fn action_transition(&self, config: &ATNConfig, t: &ActionTransition) -> ATNConfig {
        config.cloned(self.atn().states[t.target].as_ref())
    }

    fn precedence_transition(&self,
                             config: &ATNConfig,
                             pt: &PrecedencePredicateTransition,
                             collect_predicates: bool,
                             in_context: bool,
                             full_ctx: bool,
    ) -> ATNConfig { unimplemented!() }

    fn pred_transition(&self,
                       config: &ATNConfig,
                       pt: &PredicateTransition,
                       collect_predicates: bool,
                       in_context: bool,
                       full_ctx: bool,
    ) -> ATNConfig { unimplemented!() }

    fn rule_transition(&self, config: &ATNConfig, t: &RuleTransition) -> ATNConfig {
        let new_ctx = PredictionContext::new_singleton(
            config.get_context().cloned().map(Box::new),
            t.follow_state as isize,
        );
        config.cloned_with_new_ctx(self.atn().states[t.target].as_ref(), Some(new_ctx))
    }
    //
    fn get_conflicting_alts(&self, configs: &ATNConfigSet) -> BitSet { unimplemented!() }
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
    fn no_viable_alt(&self, local: &mut Local, configs: &ATNConfigSet, start_index: isize)
                     -> ANTLRError {
        unimplemented!()
    }

    fn get_unique_alt(&self, configs: &ATNConfigSet) -> isize {
        let mut alt = INVALID_ALT;
        for c in configs.get_items() {
            if alt == INVALID_ALT {
                alt = c.get_alt()
            } else if c.get_alt() != alt {
                return INVALID_ALT
            }
        }

        alt
    }

    fn add_dfaedge(&self, from: &mut DFAState, t: isize, to: DFAStateRef) -> DFAStateRef {
        if t < -1 || t > self.atn().max_token_type { return to }
        if from.edges.is_empty() {
            from.edges.resize(self.atn().max_token_type as usize + 2, 0);
        }
        from.edges[(t + 1) as usize] = to;

        to
    }

    fn add_dfastate(&self, dfa: &DFA, mut dfastate: DFAState) -> DFAStateRef {
        if dfastate.state_number == ERROR_DFA_STATE_REF {
            return ERROR_DFA_STATE_REF;
        }

        let key = dfastate.default_hash();
        let mut states = dfa.states.write().unwrap();
        let mut new_hash = key;

        let a = *dfa.states_map.write().unwrap()
            .entry(key)
            .or_insert_with(|| {
                let last = states.deref().len();
                dfastate.state_number = last;
                if !dfastate.configs.read_only() {
                    dfastate.configs.optimize_configs(self);
                    dfastate.configs.set_read_only(true);
                    new_hash = dfastate.default_hash();
                }

                states.push(dfastate);
                last
            });

        if key != new_hash {
            dfa.states_map.write().unwrap().insert(new_hash, a);
        }
        a
    }

    fn report_attempting_full_context(&self,
                                      dfa: &DFA,
                                      conflicting_alts: &BitSet,
                                      configs: &ATNConfigSet,
                                      start_index: isize,
                                      stop_index: isize,
    ) {
        //todo
    }

    fn report_context_sensitivity(&self, dfa: &DFA, prediction: isize, configs: &ATNConfigSet,
                                  start_index: isize, stop_index: isize) {
        unimplemented!()
    }

    fn report_ambiguity(&self, dfa: &DFA, start_index: isize, stop_index: isize,
                        exact: bool, ambig_alts: &BitSet, configs: &ATNConfigSet) { unimplemented!() }
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
//
//
//}
//