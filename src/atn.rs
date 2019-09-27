use std::collections::HashMap;
use crate::rule_context::RuleContext;
use crate::interval_set::IntervalSet;
use crate::atn_state::ATNState;
use crate::atn_type::ATNType;
use crate::atn_state::ATNStateRef;
use crate::lexer_action::LexerAction;

pub struct ATN {
    pub decision_to_state: Vec<ATNStateRef>,

    pub grammar_type: ATNType,

    pub lexer_actions: Vec<LexerAction>,

    pub max_token_type: isize,

    pub mode_name_to_start_state: HashMap<String, ATNStateRef>,

    pub mode_to_start_state: Vec<ATNStateRef>,

    pub rule_to_start_state: Vec<ATNStateRef>,

    pub rule_to_stop_state: Vec<ATNStateRef>,

    pub rule_to_token_type: Vec<isize>,

    pub states: Vec<Box<ATNState>>,
}

impl ATN {
    pub fn new_atn(grammarType: ATNType, maxTokenType: isize) -> ATN {
        ATN {
            decision_to_state: Vec::new(),
            grammar_type: grammarType,
            lexer_actions: vec![],
            max_token_type: maxTokenType,
            mode_name_to_start_state: HashMap::new(),
            mode_to_start_state: Vec::new(),
            rule_to_start_state: Vec::new(),
            rule_to_stop_state: Vec::new(),
            rule_to_token_type: Vec::new(),
            states: Vec::new(),
        }
    }

    fn next_tokens_in_context(&self, _s: ATNStateRef, _ctx: &RuleContext) -> IntervalSet {
        unimplemented!()
    }

    fn next_tokens_no_context(&self, _s: ATNStateRef) -> IntervalSet {
        unimplemented!()
    }

    pub fn next_tokens(&self, _s: &dyn ATNState) -> IntervalSet {
        unimplemented!()
    }

    fn next_tokens_with_ctx(&self, _s: &dyn ATNState, _ctx: &dyn RuleContext) -> IntervalSet {
        unimplemented!()
    }

    pub fn add_state(&mut self, _state: Box<ATNState>) {
        self.states.push(_state)
    }

    fn remove_state(&self, _state: ATNStateRef) {
        unimplemented!()
    }

    fn define_decision_state(&self, _s: ATNStateRef) -> isize {
        unimplemented!()
    }

    pub fn get_decision_state(&self, decision: usize) -> ATNStateRef {
        self.decision_to_state[decision]
    }

    fn get_expected_tokens(&self, _stateNumber: isize, _ctx: &RuleContext) -> IntervalSet {
        unimplemented!()
    }
}
