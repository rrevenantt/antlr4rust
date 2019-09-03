use std::collections::{HashMap, HashSet};
use crate::dfa_state::{DFAState, DFAStateRef};

use std::sync::{Arc, RwLock};

use crate::atn_config_set::ATNConfigSet;
use crate::atn_state::{ATNDecisionState, ATNState, ATNStateRef, ATNStateType};
use crate::atn::ATN;
use crate::prediction_context::MurmurHasherBuilder;
use std::pin::Pin;
use std::task::RawWaker;

pub struct DFA {
    atn_start_state: ATNStateRef,

    decision: isize,

    /// Set of all dfa states.
    pub states: RwLock<Vec<DFAState>>,

    // for faster duplicate search
    //TODO maybe hashmap for states will be enough
    // i think DFAState.edges can contain references to its elements
    pub states_map: RwLock<HashMap</*DFAState hash*/ u64, DFAStateRef>>,
    //    states_mu sync.RWMutex
    pub s0: RwLock<Option<Box<DFAStateRef>>>,
    //    s0_mu sync.RWMutex
    precedence_dfa: bool,
}

//struct DFAStateRef{
//}

impl DFA {
    pub fn new(atn: Arc<ATN>, atn_start_state: ATNStateRef, decision: isize) -> DFA {
        let mut dfa = DFA {
            atn_start_state,
            decision,
            states: RwLock::new(Vec::new()),
            //            states_map: RwLock::new(HashMap::new()),
            states_map: RwLock::new(HashMap::new()),
            s0: RwLock::new(None),
            precedence_dfa: false,
        };

        // to indicate null
        dfa.states.write().unwrap().push(DFAState::new_dfastate(
            usize::max_value(),
            Box::new(ATNConfigSet::new_base_atnconfig_set(true)),
        ));
        if let ATNStateType::DecisionState {
            state: ATNDecisionState::PlusLoopBack,
            ..
        } = atn.states[atn_start_state].get_state_type()
        {
            dfa.precedence_dfa = true;
            let mut precedence_state = DFAState::new_dfastate(
                usize::max_value(),
                Box::new(ATNConfigSet::new_base_atnconfig_set(true)),
            );
            precedence_state.edges = vec![];
            precedence_state.is_accept_state = false;
            precedence_state.requires_full_context = false;

            dfa.s0 = RwLock::new(Some(Box::new(precedence_state.state_number)));
            dfa.states.write().unwrap().push(precedence_state)
        }
        dfa
    }

    fn get_precedence_start_state(&self, _precedence: isize) -> &DFAState {
        unimplemented!()
    }

    fn set_precedence_start_state(&self, _precedence: isize, _startState: &DFAState) {
        unimplemented!()
    }

    fn set_precedence_dfa(&self, _precedenceDfa: bool) {
        unimplemented!()
    }

    fn get_s0(&self) -> &DFAState {
        unimplemented!()
    }

    fn set_s0(&mut self, _s: &DFAState) {
        unimplemented!()
    }

    fn get_state(&self, _id: isize) -> Option<&DFAState> {
        unimplemented!()
    }

    fn set_state(&self, _id: isize, _state: &DFAState) {
        unimplemented!()
    }

    fn num_states(&self) -> isize {
        unimplemented!()
    }

    fn to_lexer_String(&self) -> String {
        unimplemented!()
    }
}
