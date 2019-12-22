use std::ops::Deref;

use crate::atn_deserializer::cast;
use crate::atn_simulator::IATNSimulator;
use crate::atn_state::*;
use crate::atn_state::ATNStateType;
use crate::dfa::ScopeExt;
use crate::errors::{ANTLRError, FailedPredicateError, InputMisMatchError, NoViableAltError, RecognitionError};
use crate::interval_set::IntervalSet;
use crate::parser::Parser;
use crate::token::{OwningToken, Token, TOKEN_EOF, TOKEN_EPSILON};
use crate::transition::RuleTransition;
use crate::transition::TransitionType::TRANSITION_RULE;

pub trait ErrorStrategy {
    fn reset(&mut self, recognizer: &mut dyn Parser);
    fn recover_inline(&mut self, recognizer: &mut dyn Parser) -> Result<OwningToken, ANTLRError>;
    fn recover(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError);
    fn sync(&mut self, recognizer: &mut dyn Parser) -> Result<(), ANTLRError>;
    fn in_error_recovery_mode(&mut self, recognizer: &mut dyn Parser) -> bool;
    fn report_error(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError);
    fn report_match(&mut self, recognizer: &mut dyn Parser);
}

pub struct DefaultErrorStrategy {
    error_recovery_mode: bool,
    last_error_index: isize,
    last_error_states: Option<IntervalSet>,
}


impl DefaultErrorStrategy {
    pub fn new() -> DefaultErrorStrategy {
        DefaultErrorStrategy {
            error_recovery_mode: false,
            last_error_index: -1,
            last_error_states: None
        }
    }

    fn begin_error_condition(&mut self, recognizer: &dyn Parser) {
        self.error_recovery_mode = true;
    }

    fn end_error_condition(&mut self, recognizer: &dyn Parser) {
        self.error_recovery_mode = false;
        self.last_error_index = -1;
        self.last_error_states = None;
    }

    fn report_no_viable_alternative(&self, recognizer: &mut dyn Parser, e: &NoViableAltError) -> String {
        let input = if e.start_token.token_type == TOKEN_EOF { "<EOF>".to_owned() } else { recognizer.get_input_stream().get_text_from_tokens(&e.start_token, &e.offending_token) };

        format!("no viable alternative at input {}", input)
    }

    fn report_input_mismatch(&self, recognizer: &dyn Parser, e: &InputMisMatchError) -> String {
        format!("mismatched input {} expecting {}",
                self.get_token_error_display(&e.base.offending_token),
                e.base.get_expected_tokens(recognizer).to_token_string(recognizer.get_vocabulary()))
    }

    fn report_failed_predicate(&self, recognizer: &dyn Parser, e: &FailedPredicateError) -> String {
        format!("rule {} {}",
                recognizer.get_rule_names()[recognizer.get_parser_rule_context().get_rule_index()],
                e.base.message)
    }

    fn report_unwanted_token(&mut self, recognizer: &mut dyn Parser) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let expecting = self.get_expected_tokens(recognizer);
        let expecting = expecting.to_token_string(recognizer.get_vocabulary());
        let t = recognizer.get_current_token();
        let token_name = self.get_token_error_display(t);
        let msg = format!("extraneous input {} expecting {}", token_name, expecting);
        let t = t.get_token_index();
        recognizer.notify_error_listeners(msg, Some(t), None);
    }

    fn report_missing_token(&mut self, recognizer: &mut dyn Parser) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let expecting = self.get_expected_tokens(recognizer);
        let expecting = expecting.to_token_string(recognizer.get_vocabulary());
        let t = recognizer.get_current_token();
        let token_name = self.get_token_error_display(t);
        let msg = format!("missing {} at {}",
                          expecting,
                          self.get_token_error_display(t)
        );
        let t = t.get_token_index();
        recognizer.notify_error_listeners(msg, Some(t), None);
    }

    fn single_token_insertion(&mut self, recognizer: &mut dyn Parser) -> bool {
        let current_token = recognizer.get_input_stream().la(1);

        let atn = recognizer.get_interpreter().atn();
        let current_state = atn.states[recognizer.get_state() as usize].as_ref();
        let next = current_state.get_transitions().first().unwrap().get_target();
        let expect_at_ll2 = atn.next_tokens_in_ctx(atn.states[next].as_ref(), Some(recognizer.get_parser_rule_context().deref()));
        if expect_at_ll2.contains(current_token) {
            self.report_missing_token(recognizer);
            return true;
        }
        false
    }

    fn single_token_deletion<'a>(&mut self, recognizer: &'a mut dyn Parser) -> Option<&'a dyn Token> {
        let next_token_type = recognizer.get_input_stream().la(2);
        let expecting = self.get_expected_tokens(recognizer);
        println!("expecting {}", expecting.to_token_string(recognizer.get_vocabulary()));
        if expecting.contains(next_token_type) {
            self.report_unwanted_token(recognizer);
            recognizer.consume();
            self.report_match(recognizer);
            let matched_symbol = recognizer.get_current_token();
            return Some(matched_symbol);
        }
        None
    }

    fn get_missing_symbol(&self, recognizer: &mut dyn Parser) -> OwningToken { unimplemented!() }

    fn get_expected_tokens(&self, recognizer: &dyn Parser) -> IntervalSet {
        recognizer.get_expected_tokens()
    }

    fn get_token_error_display(&self, t: &dyn Token) -> String { unimplemented!() }

    fn escape_wsand_quote(&self, s: String) -> String { unimplemented!() }

    fn get_error_recovery_set(&self, recognizer: &dyn Parser) -> IntervalSet {
        let atn = recognizer.get_interpreter().atn();
        let mut ctx = Some(recognizer.get_parser_rule_context().clone());
        let mut recover_set = IntervalSet::new();
        while let Some(c) = ctx {
            if c.get_invoking_state() < 0 { break }

            let invoking_state = atn.states[c.get_invoking_state() as usize].as_ref();
            let tr = invoking_state.get_transitions().first().unwrap().as_ref();
            assert_eq!(tr.get_serialization_type(), TRANSITION_RULE);
            let tr = unsafe { cast::<RuleTransition>(tr) };
            let follow = atn.next_tokens(atn.states[tr.follow_state].as_ref());
            recover_set.add_set(follow);
            ctx = c.peek_parent();
        }
        recover_set.remove_one(TOKEN_EPSILON);
        return recover_set
    }

    fn consume_until(&self, recognizer: &mut dyn Parser, set: &IntervalSet) {
        let mut ttype = recognizer.get_input_stream().la(1);
        while ttype != TOKEN_EOF && !set.contains(ttype) {
            recognizer.consume();
            ttype = recognizer.get_input_stream().la(1);
        }
    }
}

impl ErrorStrategy for DefaultErrorStrategy {
    fn reset(&mut self, recognizer: &mut Parser) {
        unimplemented!()
    }

    fn recover_inline(&mut self, recognizer: &mut dyn Parser) -> Result<OwningToken, ANTLRError> {
        let t = self.single_token_deletion(recognizer).map(|it| it.to_owned());
        if let Some(t) = t {
            recognizer.consume();
            return Ok(t);
        }

        if self.single_token_insertion(recognizer) {
            return Ok(self.get_missing_symbol(recognizer));
        }

        Err(ANTLRError::InputMismatchError(InputMisMatchError::new(recognizer)))
//        Err(ANTLRError::IllegalStateError("aaa".to_string()))
    }

    fn recover(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError) {
        if self.last_error_index == recognizer.get_input_stream().index()
            && self.last_error_states.is_some()
            && self.last_error_states.as_ref().unwrap().contains(recognizer.get_state()) {
            recognizer.consume()
        }

        self.last_error_index = recognizer.get_input_stream().index();
        self.last_error_states.get_or_insert(IntervalSet::new()).apply(
            |x| x.add_one(recognizer.get_state())
        );
        let follow_set = self.get_error_recovery_set(recognizer);
        self.consume_until(recognizer, &follow_set);
    }

    fn sync(&mut self, recognizer: &mut dyn Parser) -> Result<(), ANTLRError> {
        if self.in_error_recovery_mode(recognizer) { return Ok(()); }
        let next = recognizer.get_input_stream().la(1);
        let state = recognizer.get_interpreter().atn().states[recognizer.get_state() as usize].as_ref();

        let next_tokens = recognizer.get_interpreter().atn().next_tokens(state);

        if next_tokens.contains(next) {
            return Ok(());
        }

        if next_tokens.contains(TOKEN_EPSILON) {
            return Ok(());
        }

        match state.get_state_type_id() {
            ATNSTATE_BLOCK_START |
            ATNSTATE_PLUS_BLOCK_START |
            ATNSTATE_STAR_BLOCK_START |
            ATNSTATE_STAR_LOOP_ENTRY =>
                if self.single_token_deletion(recognizer).is_none() {
                    return Err(ANTLRError::InputMismatchError(InputMisMatchError::new(recognizer)));
                }
            ATNSTATE_PLUS_LOOP_BACK |
            ATNSTATE_STAR_LOOP_BACK => {
                self.report_unwanted_token(recognizer);
                let mut expecting = recognizer.get_expected_tokens();
                expecting.add_set(&self.get_error_recovery_set(recognizer));
                self.consume_until(recognizer, &expecting);
            }
            _ => { panic!("invalid ANTState type id") }
        }

        Ok(())
    }

    fn in_error_recovery_mode(&mut self, recognizer: &mut Parser) -> bool {
        self.error_recovery_mode
    }

    fn report_error(&mut self, recognizer: &mut Parser, e: &ANTLRError) {
        if self.in_error_recovery_mode(recognizer) { return; }

        self.begin_error_condition(recognizer);
        let msg = match e {
            ANTLRError::NoAltError(e) => self.report_no_viable_alternative(recognizer, e),
            ANTLRError::InputMismatchError(e) => self.report_input_mismatch(recognizer, e),
            ANTLRError::PredicateError(e) => self.report_failed_predicate(recognizer, e),
            _ => e.to_string()
        };
        let offending_token_index = e.get_offending_token().map(|it| it.get_token_index());
        recognizer.notify_error_listeners(e.to_string(), offending_token_index, Some(&e))
    }

    fn report_match(&mut self, recognizer: &mut Parser) {
        self.end_error_condition(recognizer);
        //println!("matched token succesfully {}", recognizer.get_input_stream().la(1))
    }
}

//    pub struct BailErrorStrategy {
//    base: DefaultErrorStrategy,
//    }
//
//impl BailErrorStrategy{
//    fn new_bail_error_strategy() -> * BailErrorStrategy { unimplemented!() }
//
//    fn recover(&self, recognizer: Parser, e: RecognitionError) { unimplemented!() }
//
//    fn recover_inline(&self, recognizer: Parser) -> Token { unimplemented!() }
//
//    fn sync(&self, recognizer: Parser) { unimplemented!() }
//}
 