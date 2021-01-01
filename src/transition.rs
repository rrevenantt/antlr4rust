use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::fmt::Debug;

use crate::atn_state::ATNStateRef;
use crate::interval_set::IntervalSet;
use crate::lexer::{LEXER_MAX_CHAR_VALUE, LEXER_MIN_CHAR_VALUE};
use crate::semantic_context::SemanticContext;

const _TRANSITION_NAMES: [&'static str; 11] = [
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

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
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

// todo remove trait because it is too slow
/// Transition between ATNStates
pub trait Transition: Sync + Send + Debug + Any {
    fn get_target(&self) -> ATNStateRef;
    fn set_target(&mut self, s: ATNStateRef);
    fn is_epsilon(&self) -> bool { false }
    fn get_label(&self) -> Option<Cow<'_, IntervalSet>> { None }
    fn get_serialization_type(&self) -> TransitionType;
    fn matches(&self, symbol: isize, min_vocab_symbol: isize, max_vocab_symbol: isize) -> bool;
    fn get_predicate(&self) -> Option<SemanticContext> { None }
    fn get_reachable_target(&self, symbol: isize) -> Option<ATNStateRef> {
        //        println!("reachable target called on {:?}", self);
        if self.matches(symbol, LEXER_MIN_CHAR_VALUE, LEXER_MAX_CHAR_VALUE) {
            return Some(self.get_target());
        }
        None
    }
}

impl dyn Transition {
    #[inline]
    pub fn cast<T: Transition>(&self) -> &T {
        assert_eq!(self.type_id(), TypeId::of::<T>());
        unsafe { &*(self as *const dyn Transition as *const T) }
    }
}

#[derive(Debug)]
pub struct AtomTransition {
    pub target: ATNStateRef,
    pub label: isize,
}

impl Transition for AtomTransition {
    fn get_target(&self) -> ATNStateRef { self.target }

    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn get_label(&self) -> Option<Cow<'_, IntervalSet>> {
        let mut r = IntervalSet::new();
        r.add_one(self.label);
        Some(Cow::Owned(r))
    }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_ATOM }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
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
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn is_epsilon(&self) -> bool { true }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_RULE }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct EpsilonTransition {
    pub target: ATNStateRef,
    pub outermost_precedence_return: isize,
}

impl Transition for EpsilonTransition {
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn is_epsilon(&self) -> bool { true }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_EPSILON }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
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
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn get_label(&self) -> Option<Cow<'_, IntervalSet>> {
        let mut r = IntervalSet::new();
        r.add_range(self.start, self.stop);
        Some(Cow::Owned(r))
    }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_RANGE }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
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
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn is_epsilon(&self) -> bool { true }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_ACTION }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct SetTransition {
    pub target: ATNStateRef,
    pub set: IntervalSet,
}

impl Transition for SetTransition {
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn get_label(&self) -> Option<Cow<'_, IntervalSet>> { Some(Cow::Borrowed(&self.set)) }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_SET }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
        self.set.contains(_symbol)
    }
}

#[derive(Debug)]
pub struct NotSetTransition {
    pub target: ATNStateRef,
    pub set: IntervalSet,
}

impl Transition for NotSetTransition {
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn get_label(&self) -> Option<Cow<'_, IntervalSet>> { Some(Cow::Borrowed(&self.set)) }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_NOTSET }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
        _symbol >= _min_vocab_symbol && _symbol <= _max_vocab_symbol && !self.set.contains(_symbol)
    }
}

#[derive(Debug)]
pub struct WildcardTransition {
    pub target: ATNStateRef,
}

impl Transition for WildcardTransition {
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_WILDCARD }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
        _symbol < _max_vocab_symbol && _symbol > _min_vocab_symbol
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
    fn get_target(&self) -> ATNStateRef { self.target }

    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn is_epsilon(&self) -> bool { true }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_PREDICATE }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
        false
    }

    fn get_predicate(&self) -> Option<SemanticContext> {
        Some(SemanticContext::Predicate {
            rule_index: self.rule_index,
            pred_index: self.pred_index,
            is_ctx_dependent: self.is_ctx_dependent,
        })
    }
}

#[derive(Debug)]
pub struct PrecedencePredicateTransition {
    pub target: ATNStateRef,
    pub precedence: isize,
}

impl Transition for PrecedencePredicateTransition {
    fn get_target(&self) -> ATNStateRef { self.target }
    fn set_target(&mut self, s: ATNStateRef) { self.target = s }

    fn is_epsilon(&self) -> bool { true }

    fn get_serialization_type(&self) -> TransitionType { TransitionType::TRANSITION_PRECEDENCE }

    fn matches(&self, _symbol: isize, _min_vocab_symbol: isize, _max_vocab_symbol: isize) -> bool {
        false
    }

    fn get_predicate(&self) -> Option<SemanticContext> {
        Some(SemanticContext::Precedence(self.precedence))
    }
}
