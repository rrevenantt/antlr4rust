use crate::interval_set::IntervalSet;

use crate::atn_state::ATNStateRef;
use std::mem;
use crate::lexer::{LEXER_MAX_CHAR_VALUE, LEXER_MIN_CHAR_VALUE};
use std::fmt::Debug;
use crate::semantic_context::SemanticContext;

const TransitionNames: [&'static str; 11] = [
    "INVALID",
    "EPSILON",
    "RANGE",
    "RULE",
    "PREDICATE",
    "ATOM",
    "ACTION",
    "SET",
    "NOT_SET",
    "WILDCARD",
    "PRECEDENCE",
];

pub const TRANSITION_EPSILON: isize = 1;
pub const TRANSITION_RANGE: isize = 2;
pub const TRANSITION_RULE: isize = 3;
pub const TRANSITION_PREDICATE: isize = 4;
pub const TRANSITION_ATOM: isize = 5;
pub const TRANSITION_ACTION: isize = 6;
pub const TRANSITION_SET: isize = 7;
pub const TRANSITION_NOTSET: isize = 8;
pub const TRANSITION_WILDCARD: isize = 9;
pub const TRANSITION_PRECEDENCE: isize = 10;

#[derive(Debug)]
pub enum TransitionType {
    TRANSITION_EPSILON = 1,
    TRANSITION_RANGE,
    TRANSITION_RULE,
    TRANSITION_PREDICATE,
    TRANSITION_ATOM,
    TRANSITION_ACTION,
    TRANSITION_SET,
    TRANSITION_NOTSET,
    TRANSITION_WILDCARD,
    TRANSITION_PRECEDENCE,
}

pub trait Transition: Sync + Send + Debug {
    fn get_target(&self) -> ATNStateRef;
    fn set_target(&mut self, s: ATNStateRef);
    fn is_epsilon(&self) -> bool {
        false
    }
    fn get_label(&self) -> Option<IntervalSet> {
        None
    }
    fn get_serialization_type(&self) -> TransitionType;
    fn matches(&self, symbol: isize, minVocabSymbol: isize, maxVocabSymbol: isize) -> bool;
    fn get_predicate(&self) -> Option<SemanticContext> {
        None
    }
    fn get_reachable_target(&self, symbol: isize) -> Option<ATNStateRef> {
//        println!("reachable target called on {:?}", self);
        if self.matches(symbol, LEXER_MIN_CHAR_VALUE, LEXER_MAX_CHAR_VALUE) {
            return Some(self.get_target());
        }
        None
    }
}

//impl dyn Transition{
//    /// be sure to use `get_serialization_type` to know actual type to cast to
//    pub unsafe fn cast<T:Transition>(&self)->&T{
//        let to = mem::transmute::<&dyn Transition,std::raw::TraitObject>(self).data;
//        mem::transmute::<*mut (),&T>(to)
//    }
//
//}

//pub struct BaseTransition {
//    pub target: ATNStateRef,
//    //    is_epsilon: bool,
//    pub interval_set: IntervalSet,
//    //    serialization_type: isize,
//}

//impl BaseTransition {
//    pub fn new_base_transition(target: ATNStateRef) -> BaseTransition {
//        BaseTransition {
//            target: target,
//            interval_set: IntervalSet::new_interval_set(),
//        }
//    }
//}

//impl Transition for BaseTransition {
//    fn get_target(&self) -> ATNStateRef {
//        self.target.clone()
//    }
//
//    fn set_target(&mut self, s: ATNStateRef) {
//        self.target = s
//    }
//
//    fn get_serialization_type(&self) -> TransitionType {
//        unimplemented!()
//    }
//
//    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
//        unimplemented!()
//    }
//}

#[derive(Debug)]
pub struct AtomTransition {
    pub target: ATNStateRef,
    pub label: isize,
}

impl Transition for AtomTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }

    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_ATOM
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        _symbol == self.label
    }
}

#[derive(Debug)]
pub struct RuleTransition {
    pub target: ATNStateRef,
    pub follow_state: ATNStateRef,
    pub rule_index: isize,
    pub precedence: isize,
}

impl Transition for RuleTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn is_epsilon(&self) -> bool {
        true
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_RULE
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct EpsilonTransition {
    pub target: ATNStateRef,
    pub outermost_precedence_return: isize,
}

impl Transition for EpsilonTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn is_epsilon(&self) -> bool {
        true
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_EPSILON
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct RangeTransition {
    pub target: ATNStateRef,
    pub start: isize,
    pub stop: isize,
}

impl Transition for RangeTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_RANGE
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        _symbol >= self.start && _symbol <= self.stop
    }
}

#[derive(Debug)]
pub struct ActionTransition {
    pub target: ATNStateRef,
    pub is_ctx_dependent: bool,
    pub rule_index: isize,
    pub action_index: isize,
    pub pred_index: isize,
}

impl Transition for ActionTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn is_epsilon(&self) -> bool {
        true
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_ACTION
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct SetTransition {
    pub target: ATNStateRef,
    pub set: IntervalSet,
}

impl Transition for SetTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_SET
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        self.set.contains(_symbol)
    }
}

#[derive(Debug)]
pub struct NotSetTransition {
    pub target: ATNStateRef,
    pub set: IntervalSet,
}

impl Transition for NotSetTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_NOTSET
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        _symbol >= _minVocabSymbol && _symbol <= _maxVocabSymbol
            && !self.set.contains(_symbol)
    }
}

#[derive(Debug)]
pub struct WildcardTransition {
    pub target: ATNStateRef,
}

impl Transition for WildcardTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_WILDCARD
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        _symbol < _maxVocabSymbol && _symbol > _minVocabSymbol
    }
}

#[derive(Debug)]
pub struct PredicateTransition {
    pub target: ATNStateRef,
    pub is_ctx_dependent: bool,
    pub rule_index: isize,
    pub pred_index: isize,
}

impl Transition for PredicateTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }

    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn is_epsilon(&self) -> bool {
        true
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_PREDICATE
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct PrecedencePredicateTransition {
    pub target: ATNStateRef,
    pub precedence: isize,
}

impl Transition for PrecedencePredicateTransition {
    fn get_target(&self) -> ATNStateRef {
        self.target
    }
    fn set_target(&mut self, s: ATNStateRef) {
        self.target = s
    }

    fn get_serialization_type(&self) -> TransitionType {
        TransitionType::TRANSITION_PRECEDENCE
    }

    fn matches(&self, _symbol: isize, _minVocabSymbol: isize, _maxVocabSymbol: isize) -> bool {
        unimplemented!()
    }
}
