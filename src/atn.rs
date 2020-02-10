use std::collections::HashMap;
use std::rc::Rc;

use crate::atn_state::ATNState;
use crate::atn_state::ATNStateRef;
use crate::atn_type::ATNType;
use crate::dfa::ScopeExt;
use crate::interval_set::IntervalSet;
use crate::lexer_action::LexerAction;
use crate::ll1_analyzer::LL1Analyzer;
use crate::parser_rule_context::ParserRuleContext;
use crate::rule_context::RuleContext;
use crate::token::{TOKEN_EOF, TOKEN_EPSILON};
use crate::transition::RuleTransition;

pub const INVALID_ALT: isize = 0;

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

    pub states: Vec<Box<dyn ATNState>>,
}

impl ATN {
    pub fn new_atn(grammar_type: ATNType, max_token_type: isize) -> ATN {
        ATN {
            decision_to_state: Vec::new(),
            grammar_type,
            lexer_actions: vec![],
            max_token_type,
            mode_name_to_start_state: HashMap::new(),
            mode_to_start_state: Vec::new(),
            rule_to_start_state: Vec::new(),
            rule_to_stop_state: Vec::new(),
            rule_to_token_type: Vec::new(),
            states: Vec::new(),
        }
    }

//    fn next_tokens_in_context(&self,s: ATNStateRef, _ctx: &RuleContext) -> IntervalSet {
//        unimplemented!()
//    }
//
//    fn next_tokens_no_context(&self,s: ATNStateRef) -> IntervalSet {
//        unimplemented!()
//    }

    pub fn next_tokens<'a>(&self, s: &'a dyn ATNState) -> &'a IntervalSet {
        s.get_next_tokens_within_rule().get_or_init(|| {
            self.next_tokens_in_ctx(s, None)
                .modify_with(|r| {
//                    println!("expecting {:?}", r);
                    r.read_only = true
                }
                )
        })
    }

    pub fn next_tokens_in_ctx(&self, s: &dyn ATNState, _ctx: Option<&dyn ParserRuleContext>) -> IntervalSet {
        let analyzer = LL1Analyzer::new(self);
        analyzer.look(s, None, _ctx)
    }

    pub fn add_state(&mut self, state: Box<dyn ATNState>) {
        assert_eq!(state.get_state_number(), self.states.len());
        self.states.push(state)
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

    pub fn get_expected_tokens(&self, state_number: isize, _ctx: &Rc<dyn ParserRuleContext>) -> IntervalSet {
        let s = self.states[state_number as usize].as_ref();
        let mut following = self.next_tokens(s);
        if !following.contains(TOKEN_EPSILON) {
            return following.clone();
        }
        let mut expected = IntervalSet::new();
        expected.add_set(&following);
        expected.remove_one(TOKEN_EPSILON);
        let mut ctx = Some(Rc::clone(_ctx));

        while let Some(c) = ctx {
            if c.get_invoking_state() < 0 || !following.contains(TOKEN_EPSILON) { break }

            let invoking_state = self.states[c.get_invoking_state() as usize].as_ref();
            let tr = invoking_state.get_transitions().first().unwrap().as_ref();
            let tr = tr.cast::<RuleTransition>();
            following = self.next_tokens(self.states[tr.follow_state].as_ref());
            expected.add_set(following);
            expected.remove_one(TOKEN_EPSILON);
            ctx = c.get_parent_ctx();
        }

        if following.contains(TOKEN_EPSILON) {
            expected.add_one(TOKEN_EOF);
        }
        expected
    }
}
