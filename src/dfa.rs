use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::atn::ATN;
use crate::atn_config_set::ATNConfigSet;
use crate::atn_state::{ATNDecisionState, ATNState, ATNStateRef, ATNStateType};
use crate::dfa_serializer::DFASerializer;
use crate::dfa_state::{DFAState, DFAStateRef};
use crate::vocabulary::Vocabulary;

///Helper trait for scope management and temporary values not living long enough
pub(crate) trait ScopeExt: Sized {
    fn convert_with<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
        f(self)
    }
    fn run<T, F: FnOnce(&Self) -> T>(&self, f: F) -> T {
        f(self)
    }

    //apply
    fn modify_with<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }
    //apply_inplace
    fn apply<F: FnOnce(&mut Self)>(&mut self, f: F) -> &mut Self {
        f(self);
        self
    }

    fn drop(self) {}
}

impl<Any: Sized> ScopeExt for Any {}


pub struct DFA {
    pub atn_start_state: ATNStateRef,

    pub decision: isize,

    /// Set of all dfa states.
    pub states: RwLock<Vec<DFAState>>,

    // for faster duplicate search
    // TODO i think DFAState.edges can contain references to its elements
    pub states_map: RwLock<HashMap</*DFAState hash*/ u64, Vec<DFAStateRef>>>,
    //    states_mu sync.RWMutex
    pub s0: RwLock<Option<DFAStateRef>>,
    //    s0_mu sync.RWMutex
    is_precedence_dfa: bool,
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
            is_precedence_dfa: false,
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
            dfa.is_precedence_dfa = true;
            let mut precedence_state = DFAState::new_dfastate(
                dfa.states.read().unwrap().len(),
                Box::new(ATNConfigSet::new_base_atnconfig_set(true)),
            );
            precedence_state.edges = vec![];
            precedence_state.is_accept_state = false;
            precedence_state.requires_full_context = false;

            dfa.s0 = RwLock::new(Some(precedence_state.state_number));
            dfa.states.write().unwrap().push(precedence_state)
        }
        dfa
    }

    pub fn get_precedence_start_state(&self, _precedence: isize) -> Option<DFAStateRef> {
        if !self.is_precedence_dfa {
            panic!("dfa is supposed to be precedence here");
        }

        self.s0.read().unwrap()
            .and_then(|s0|
                self.states
                    .read().unwrap()[s0]
                    .edges
                    .get(_precedence as usize)
                    .copied()
            )
    }

    pub fn set_precedence_start_state(&self, precedence: isize, _start_state: DFAStateRef) {
        if !self.is_precedence_dfa {
            panic!("set_precedence_start_state called for not precedence dfa")
        }

        if precedence < 0 {
            return;
        }
        let precedence = precedence as usize;

        if let Some(x) = self.s0.write().unwrap().deref() {
            self.states
                .write().unwrap()[*x]
                .edges
                .apply(|edges| {
                    if edges.len() <= precedence {
                        edges.resize(precedence + 1, 0);
                    }
                    edges[precedence] = _start_state;
                });
        }
    }

    pub fn is_precedence_dfa(&self) -> bool {
        self.is_precedence_dfa
    }

    pub fn set_precedence_dfa(&mut self, precedence_dfa: bool) {
        self.is_precedence_dfa = precedence_dfa
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

    pub fn to_string(&self, vocabulary: &dyn Vocabulary) -> String {
        if self.s0.read().unwrap().is_none() { return String::new(); }

        return format!("{}", DFASerializer::new(self, &|x|
            vocabulary.get_display_name(x as isize - 1).into_owned(),
        ));
    }

    pub fn to_lexer_string(&self) -> String {
        if self.s0.read().unwrap().is_none() { return String::new(); }
        format!(
            "{}",
            DFASerializer::new(self, &|x| format!(
                "'{}'",
                char::try_from(x as u32).unwrap()
            ))
        )
    }
}
