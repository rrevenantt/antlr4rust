use crate::atn_config_set::ATNConfigSet;
use std::hash::{Hash, Hasher};
use murmur3::murmur3_32::MurmurHasher;
use crate::lexer_atn_simulator::{MAX_DFA_EDGE, MIN_DFA_EDGE};

pub struct PredPrediction {
    alt: isize,
    //    pred: SemanticContext,
}

//todo rewrite as wrapper with helper methods
pub type DFAStateRef = usize;

#[derive(Eq, PartialEq)]
pub struct DFAState {
    pub state_number: usize,
    pub configs: Box<ATNConfigSet>,
    pub edges: Vec<DFAStateRef>,
    pub is_accept_state: bool,

    pub prediction: isize,
    //    lexer_action_executor: * LexerActionExecutor,
    pub requires_full_context: bool,
    //    predicates: Vec < PredPrediction > ,
}

//impl Hash for *DFAState{
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        unsafe{ (**self).hash(state) }
//    }
//}

impl Hash for DFAState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.configs.hash(state);
    }
}

impl DFAState {
    pub fn default_hash(&self) -> u64 {
        let mut hasher = MurmurHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn new_dfastate(stateNumber: usize, configs: Box<ATNConfigSet>) -> DFAState {
        DFAState {
            state_number: stateNumber,
            configs,
            edges: Vec::with_capacity((MAX_DFA_EDGE - MIN_DFA_EDGE + 1) as usize),
            is_accept_state: false,
            prediction: 0,
            requires_full_context: false,
            //        predicates: Vec::new(),
        }
    }

    //    fn get_alt_set(&self) -> &Set { unimplemented!() }

    fn set_prediction(&self, _v: isize) {
        unimplemented!()
    }
}
