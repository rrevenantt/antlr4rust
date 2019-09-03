//lexer_atnsimulator_debug    = false
//lexer_atnsimulator_dfadebug = false
//
//lexer_atnsimulator_min_dfaedge = 0
//lexer_atnsimulator_max_dfaedge = 127
//lexer_atnsimulator_match_calls = 0

use crate::atn::ATN;
use crate::atn_simulator::BaseATNSimulator;
use crate::atn_simulator::IATNSimulator;
use crate::char_stream::CharStream;
use crate::dfa::DFA;
use crate::lexer::{Lexer, LEXER_MIN_CHAR_VALUE, LEXER_MAX_CHAR_VALUE};
use crate::prediction_context::{base_prediction_context_empty_return_state, PredictionContext,
                                PredictionContextCache};
use crate::prediction_context::EMPTY_PREDICTION_CONTEXT;
use crate::dfa_state::{DFAState, DFAStateRef};
use crate::atn_config_set::ATNConfigSet;
use crate::transition::{Transition, TransitionType, RuleTransition};
use crate::atn_state::{ATNState, ATNStateType};
use crate::atn_config::{ATNConfig, ATNConfigType, BaseATNConfig};
use crate::errors::ANTLRError;
use std::usize;
use std::ptr;
use crate::int_stream::IntStream;
use crate::int_stream::EOF;
use std::sync::{Arc, RwLockReadGuard};
use std::convert::TryFrom;
use crate::atn_state::ATNStateType::RuleStopState;
use std::ops::{Deref, DerefMut, Index};
use crate::errors::ANTLRError::LexerNoAltError;
use crate::token::TOKEN_EOF;
use crate::atn_deserializer::cast;
use std::io::{stdout, Write};

lazy_static! {
    pub static ref ERROR: DFAState = DFAState::new_dfastate(
        usize::MAX,
        Box::new(ATNConfigSet::new_base_atnconfig_set(true))
    );
}
pub const ERROR_DFA_STATE_REF: DFAStateRef = usize::MAX;

pub trait ILexerATNSimulator: IATNSimulator {
    fn reset(&self);
    fn match_token<'a>(
        &'a mut self,
        input: &'a mut CharStream,
        mode: isize,
    ) -> Result<isize, ANTLRError>;
    fn get_char_position_in_line(&self) -> isize;
    fn get_line(&self) -> isize;
    fn get_text(&self, input: &CharStream) -> String;
    fn consume(&mut self, input: &mut CharStream);
    fn recover(&mut self, _re: ANTLRError, input: &mut CharStream) {
        if input.la(1) != EOF {
            self.consume(input)
        }
    }
}

pub struct LexerATNSimulator {
    base: BaseATNSimulator,

    //    recog: &'b Lexer,
    prediction_mode: isize,
    //    merge_cache: DoubleDict,
    start_index: isize,
    line: isize,
    char_position_in_line: isize,
    mode: isize,
    prev_accept: SimState,
    match_calls: isize,
}

impl ILexerATNSimulator for LexerATNSimulator {
    fn reset(&self) {
        unimplemented!()
    }

    fn match_token<'a>(
        &'a mut self,
        input: &'a mut CharStream,
        mode: isize,
    ) -> Result<isize, ANTLRError> {
        self.mode = mode;
        let mark = input.mark();
        let result = (|| {
            self.start_index = input.index();
            self.prev_accept.reset();
            let temp = self.decision_to_dfa();
            let dfa = temp.get(mode as usize)
                .ok_or(ANTLRError::IllegalStateError("invalid mode".into()))?;

            let s0 = dfa.s0.read().unwrap().as_deref().copied();
            match s0 {
                None => self.match_atn(input),
                Some(s0) => self.exec_atn(input, s0),
                //                Err(_) => panic!("dfa rwlock error")
            }
        })();
        input.release(mark);
        result
    }

    fn get_char_position_in_line(&self) -> isize {
        self.char_position_in_line
    }

    fn get_line(&self) -> isize {
        self.line
    }

    fn get_text(&self, _input: &dyn CharStream) -> String {
        unimplemented!()
    }

    fn consume(&mut self, _input: &mut dyn CharStream) {
        let ch = _input.la(1);
        if char::try_from(ch as u32) == Ok('\n') {
            self.line += 1;
            self.char_position_in_line = 0;
        } else {
            self.char_position_in_line += 1;
        }
        _input.consume();
    }
}

impl IATNSimulator for LexerATNSimulator {
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

pub const MIN_DFA_EDGE: isize = 0;
pub const MAX_DFA_EDGE: isize = 127;

impl LexerATNSimulator {
    pub fn new_lexer_atnsimulator(
        atn: Arc<ATN>,
        decisionToDFA: Arc<Vec<DFA>>,
        sharedContextCache: Arc<PredictionContextCache>,
    ) -> LexerATNSimulator {
        LexerATNSimulator {
            base: BaseATNSimulator::new_base_atnsimulator(atn, decisionToDFA, sharedContextCache),
            prediction_mode: 0,
            start_index: 0,
            line: 0,
            char_position_in_line: 0,
            mode: 0,
            prev_accept: SimState::new(),
            match_calls: 0,
        }
    }

    fn copy_state(&self, _simulator: &mut LexerATNSimulator) {
        unimplemented!()
    }

    fn match_atn(&mut self, input: &mut CharStream) -> Result<isize, ANTLRError> {
        assert!(self.mode >= 0);
        println!("\n---start matching");
        //        let start_state = self.atn().mode_to_start_state.get(self.mode as usize).ok_or(ANTLRError::IllegalStateError("invalid mode".into()))?;
        let atn = self.atn();
        let start_state = *atn.mode_to_start_state
            .get(self.mode as usize)
            .ok_or(ANTLRError::IllegalStateError("invalid mode".into()))?;

        let _old_mode = self.mode;
        let mut s0_closure = self.compute_start_state(input, atn.states[start_state].as_ref());
        let _supress_edge = s0_closure.has_semantic_context();
        s0_closure.set_has_semantic_context(false);

        let next_state = self.add_dfastate(&mut self.get_dfa().states.write().unwrap(), s0_closure);
        //        if !_supress_edge{
        //            self.decision_to_dfa();
        //        }

        self.exec_atn(input, next_state)
    }

    //    fn get_dfa_state(&self, state_number: DFAStateRef) -> Box<dyn Deref<Target=&DFAState>>{
    //        let dfa = self.get_dfa();
    //
    //        dfa.states.read().unwrap()
    //    }

    fn exec_atn<'a>(
        &'a mut self,
        input: &'a mut dyn CharStream,
        ds0: DFAStateRef,
    ) -> Result<isize, ANTLRError> {
        //        if self.get_dfa().states.read().unwrap().get(ds0).unwrap().is_accept_state{
        self.capture_sim_state(input, ds0);
        //        }

        let mut symbol = input.la(1);
        let mut s = ds0;
        loop {
            let target = self.get_existing_target_state(s, symbol);
            let target = target.unwrap_or(self.compute_target_state(input, s, symbol));
            //              let target = dfastates.deref().get(s).unwrap() ;x

            if target == ERROR_DFA_STATE_REF {
                break;
            }

            if symbol != EOF {
                self.consume(input)
            }

            if self.capture_sim_state(input, target) {
                if symbol == EOF {
                    break;
                }
            }

            symbol = input.la(1);

            s = target;
        }
        let last = self.get_dfa().states.read().unwrap().get(s).unwrap();

        self.fail_or_accept(input, symbol)
    }

    fn get_existing_target_state(&self, _s: DFAStateRef, t: isize) -> Option<DFAStateRef> {
        if t < MIN_DFA_EDGE || t > MAX_DFA_EDGE {
            return None;
        }

        self.get_dfa()
            .states
            .read().unwrap()
            .get(_s).unwrap()
            .edges
            .get((t - MIN_DFA_EDGE) as usize)
            .and_then(|x| match x {
                0 => None,
                x => Some(x),
            })
            .copied()
    }

    fn compute_target_state(&self, _input: &dyn CharStream, _s: DFAStateRef, _t: isize) -> DFAStateRef {
        let mut states = self.get_dfa().states.write().unwrap();

        let mut reach = ATNConfigSet::new_base_atnconfig_set(true);
        self.get_reachable_config_set(
            &states,
            _input,
            &states.get(_s).unwrap().configs,
            &mut reach,
            _t,
        );

        if reach.is_empty() {
            if !reach.has_semantic_context() {
                self.add_dfaedge(states.get_mut(_s).unwrap(), _t, ERROR_DFA_STATE_REF);
            }
            return ERROR_DFA_STATE_REF;
        }

        let supress_edge = reach.has_semantic_context();
        reach.set_has_semantic_context(false);
        let to = self.add_dfastate(&mut states, Box::new(reach));
        if !supress_edge {
            let from = states.get_mut(_s).unwrap();
            self.add_dfaedge(from, _t, to);
        }
        println!("target state computed from {:?} to {:?} on symbol {}", _s, to, char::try_from(_t as u32).unwrap());

        to
        //        states.get(to).unwrap()
    }

    fn get_reachable_config_set<T>(
        &self,
        states: &T,
        _input: &dyn CharStream,
        _closure: &ATNConfigSet,
        _reach: &mut ATNConfigSet,
        _t: isize,
    ) where
        T: Deref<Target=Vec<DFAState>>,
    {
        let mut skip_alt = 0;
        for config in _closure.get_items() {
            println!("updating reachable configset from state {}", config.get_state());
//            stdout().flush();
            let current_alt_reached_accept_state = config.get_alt() == skip_alt;
            if current_alt_reached_accept_state {
                if let ATNConfigType::LexerATNConfig {
                    passed_through_non_greedy_decision: true,
                } = config.get_type()
                {
                    continue;
                }
            }
            let atn_state = self.atn().states[config.get_state()].as_ref();
            for tr in atn_state.get_transitions() {
                if let Some(target) = tr.get_reachable_target(_t) {
                    //todo lexer executor
                    let new = BaseATNConfig::new_lexer_atnconfig3(config, self.atn().states[target].as_ref());
                    if self.closure(
                        _input,
                        new,
                        _reach,
                        current_alt_reached_accept_state,
                        true,
                        _t == EOF,
                    ) {
                        skip_alt = config.get_alt();
                        break;
                    }
                }
            }
        }
    }

//    fn get_reachable_target<T>(&self, states: &T, _trans: &Transition, _t: isize) -> &ATNState
//    where
//        T: Deref<Target = Vec<DFAState>>,
//    {
//        unimplemented!()
//    }

    fn fail_or_accept(&mut self, _input: &mut dyn CharStream, _t: isize) -> Result<isize, ANTLRError> {
        println!("fail_or_accept");
        if let Some(state) = self.prev_accept.dfa_state {
            //todo lexer action executor
            let dfa_state_prediction = self.get_dfa()
                .states
                .read()
                .unwrap()
                .get(state)
                .unwrap()
                .prediction;
            println!("accepted, prediction = {}", dfa_state_prediction);
            self.accept(_input);
            Ok(dfa_state_prediction)
        } else {
            if _t == EOF && _input.index() == self.start_index {
                return Ok(TOKEN_EOF);
            }
            Err(LexerNoAltError {
                start_index: self.start_index,
            })
        }
    }

    fn accept(&mut self, input: &mut CharStream) {
        input.seek(self.prev_accept.index);
        self.line = self.prev_accept.line;
        self.char_position_in_line = self.prev_accept.column;
    }

    fn compute_start_state(&self, _input: &CharStream, _p: &ATNState) -> Box<ATNConfigSet> {
        //        let initial_context = &EMPTY_PREDICTION_CONTEXT;
        let mut config_set = ATNConfigSet::new_base_atnconfig_set(true);

        for (i, tr) in _p.get_transitions().iter().enumerate() {
            let target = tr.get_target();
            let atn_config = BaseATNConfig::new_lexer_atnconfig6(
                target,
                (i + 1) as isize,
                EMPTY_PREDICTION_CONTEXT.clone(),
            );
            self.closure(_input, atn_config, &mut config_set, false, false, false);
        }
        println!("start_state computed {:?}", _p.get_state_type());

        Box::new(config_set)
    }

    fn closure(
        &self,
        _input: &dyn CharStream,
        mut config: BaseATNConfig,
        _configs: &mut ATNConfigSet,
        mut _current_alt_reached_accept_state: bool,
        _speculative: bool,
        _treatEOFAsEpsilon: bool,
    ) -> bool {
        //        let config = &config;
        let atn = self.atn();
        let state = atn.states[config.get_state()].as_ref();
        println!("closure called on state {} {:?}", state.get_state_number(), state.get_state_type());

        if let ATNStateType::RuleStopState {} = state.get_state_type() {
            println!("reached rulestopstate");
            if config.get_context().map(|x| x.has_empty_path()) != Some(false) {
                if config.get_context().map(|x| x.is_empty()) != Some(false) {
                    _configs.add(Box::new(config));
                    return true;
                } else {
                    _configs.add(Box::new(BaseATNConfig::new_lexer_atnconfig2(
                        &config,
                        state,
                        Some(EMPTY_PREDICTION_CONTEXT.clone()),
                    )));
                    _current_alt_reached_accept_state = true
                }
            }

            if config.get_context().map(|x| x.has_empty_path()) == Some(false) {
                let ctx = config.get_context().unwrap();
                for i in 0..ctx.length() {
                    if ctx.get_return_state(i) != base_prediction_context_empty_return_state {
                        //                        let newCtx =
                        unimplemented!()
                    }
                }
            }

            return _current_alt_reached_accept_state;
        }

        if !state.has_epsilon_only_transitions() {
            if !_current_alt_reached_accept_state
                || config.config_type == (ATNConfigType::LexerATNConfig {
                passed_through_non_greedy_decision: true,
            }) {
                _configs.add(Box::new(config.clone()));
            }
        }

        let state = atn.states[config.get_state()].as_ref();

        for tr in state.get_transitions() {
            let c = self.get_epsilon_target(
                _input,
                &mut config,
                tr.as_ref(),
                _configs,
                _speculative,
                _treatEOFAsEpsilon,
            );

            if let Some(c) = c {
                _current_alt_reached_accept_state = self.closure(
                    _input,
                    c,
                    _configs,
                    _current_alt_reached_accept_state,
                    _speculative,
                    _treatEOFAsEpsilon,
                );
            }
        }

        _current_alt_reached_accept_state
    }

    fn get_epsilon_target(
        &self,
        _input: &dyn CharStream,
        _config: &mut BaseATNConfig,
        _trans: &dyn Transition,
        _configs: &ATNConfigSet,
        _speculative: bool,
        _treatEOFAsEpsilon: bool,
    ) -> Option<BaseATNConfig> {
        let mut result = None;
        let target = self.atn().states.get(_trans.get_target()).unwrap().as_ref();
        println!("epsilon target for {:?} is {:?}", _trans, target.get_state_type());
        match _trans.get_serialization_type() {
            TransitionType::TRANSITION_EPSILON => {
                result = Some(BaseATNConfig::new_lexer_atnconfig3(_config, target));
            }
            TransitionType::TRANSITION_RULE => {
                let rt = unsafe { cast::<RuleTransition>(_trans) };
                println!("rule transition follow state{}", rt.follow_state);
                let pred_ctx = PredictionContext::new_singleton_prediction_context(
                    Some(Box::new(_config.take_context())),
                    rt.follow_state as isize,
                );
                result = Some(BaseATNConfig::new_lexer_atnconfig2(_config, target, Some(pred_ctx)));
            }
            TransitionType::TRANSITION_PREDICATE => {
                unimplemented!();
            }
            TransitionType::TRANSITION_ACTION => {
                unimplemented!();
            }
            TransitionType::TRANSITION_RANGE => {
                println!("TransitionType::TRANSITION_RANGE");
            }
            TransitionType::TRANSITION_SET |
            TransitionType::TRANSITION_WILDCARD |
            TransitionType::TRANSITION_ATOM =>
                if _treatEOFAsEpsilon {
                    if _trans.matches(EOF, LEXER_MIN_CHAR_VALUE, LEXER_MAX_CHAR_VALUE) {
                        let target = self.atn().states[_trans.get_target()].as_ref();
                        result = Some(BaseATNConfig::new_lexer_atnconfig3(_config, target));
                    }
                },
            TransitionType::TRANSITION_NOTSET => {
//                println!("TransitionType::TRANSITION_NOTSET !!!!!!!!!!!!!");
            }
            TransitionType::TRANSITION_PRECEDENCE => {
                panic!("precedence predicates are not supposed to be in lexer");
            }
        }

        result
    }

    fn evaluate_predicate(
        &self,
        _input: &dyn CharStream,
        _ruleIndex: isize,
        _predIndex: isize,
        _speculative: bool,
    ) -> bool {
        unimplemented!()
    }

    fn capture_sim_state(&mut self, input: &CharStream, dfa_state: DFAStateRef) -> bool {
        if self.get_dfa()
            .states
            .read().unwrap()
            .get(dfa_state).unwrap()
            .is_accept_state
        {
            self.prev_accept = SimState {
                index: input.index(),
                line: self.line,
                column: self.char_position_in_line,
                dfa_state: Some(dfa_state),
            };
            return true;
        }
        false
    }

    fn add_dfaedge(&self, _from: &mut DFAState, t: isize, _to: DFAStateRef) {
        if t < MIN_DFA_EDGE || t > MAX_DFA_EDGE {
            return;
        }

        if _from.edges.len() < (MAX_DFA_EDGE - MIN_DFA_EDGE + 1) as usize {
            _from
                .edges
                .resize((MAX_DFA_EDGE - MIN_DFA_EDGE + 1) as usize, 0);
        }

        _from.edges[t as usize] = _to;
    }

    fn add_dfastate<T>(&self, states: &mut T, _configs: Box<ATNConfigSet>) -> DFAStateRef
        where
            T: DerefMut<Target=Vec<DFAState>>,
    {
        let rule_index = _configs
            .get_items()
            .find(|c| {
                if let RuleStopState = self.atn().states[c.get_state()].get_state_type() {
                    true
                } else {
                    false
                }
            })
            .map(|c| {
                let rule_index = self.atn().states[c.get_state()].get_rule_index();

                self.atn().rule_to_token_type[rule_index]
            });

        let mut dfastate = DFAState::new_dfastate(usize::MAX, _configs);
        if let Some(i) = rule_index {
            println!("accepted");
            dfastate.prediction = i;
            dfastate.is_accept_state = true;
        }

        let dfa = self.get_dfa();
        let key = dfastate.default_hash();
        let dfastate_index = *dfa.states_map.write().unwrap().entry(key).or_insert_with(|| {
            dfastate.state_number = states.deref().len();
            dfastate.configs.set_read_only(true);
            let i = dfastate.state_number;
            println!("inserting new DFA state {} with size {}", i, dfastate.configs.length());
            states.deref_mut().push(dfastate);
            i
        });

        println!("new DFA state {}", dfastate_index);

        //        dfa.states.write().unwrap().get_mut(*dfastate_index).unwrap()
        dfastate_index
    }

    fn get_dfa(&self) -> &DFA {
        self.decision_to_dfa().get(self.mode as usize).unwrap()
    }

    fn get_token_name(&self, _tt: isize) -> String {
        unimplemented!()
    }

    fn reset_sim_state(_sim: &mut SimState) {
        unimplemented!()
    }
}

pub struct SimState {
    index: isize,
    line: isize,
    column: isize,
    dfa_state: Option<usize>,
}

impl SimState {
    pub fn new() -> SimState {
        SimState {
            index: -1,
            line: 0,
            column: -1,
            dfa_state: None,
        }
    }

    fn reset(&mut self) {
        self.index = -1;
        self.line = 0;
        self.column = -1;
        self.dfa_state = None;
    }
}
