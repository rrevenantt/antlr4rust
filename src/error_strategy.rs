use crate::errors::{RecognitionError, ANTLRError, InputMisMatchError};
use crate::token::{Token, TOKEN_EPSILON};
use crate::parser::Parser;
use crate::atn_simulator::IATNSimulator;
use crate::atn_state::ATNStateType;
use crate::atn_state::*;

pub trait ErrorStrategy {
    fn reset(&mut self, recognizer: &mut dyn Parser);
    fn recover_inline(&mut self, recognizer: &mut dyn Parser) -> Result<&dyn Token, ANTLRError>;
    fn recover(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError);
    fn sync(&mut self, recognizer: &mut dyn Parser) -> Result<(), ANTLRError>;
    fn in_error_recovery_mode(&mut self, recognizer: &mut dyn Parser) -> bool;
    fn report_error(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError);
    fn report_match(&mut self, recognizer: &mut dyn Parser);
}

pub struct DefaultErrorStrategy {
    error_recovery_mode: bool,
    last_error_index: isize,
//    last_error_states: * IntervalSet,
}


impl DefaultErrorStrategy {
    pub(crate) fn new() -> DefaultErrorStrategy {
        DefaultErrorStrategy {
            error_recovery_mode: false,
            last_error_index: -1,
        }
    }

    //    fn begin_error_condition(&self, recognizer: Parser) { unimplemented!() }
//
//    fn end_error_condition(&self, recognizer: Parser) { unimplemented!() }
//
//    fn report_no_viable_alternative(&self, recognizer: Parser, e: * NoViableAltError) { unimplemented!() }
//
//    fn report_input_mis_match(&self, recognizer: Parser, e: * InputMisMatchError) { unimplemented!() }
//
//    fn report_failed_predicate(&self, recognizer: Parser, e: * FailedPredicateError) { unimplemented!() }
//
//    fn report_unwanted_token(&self, recognizer: Parser) { unimplemented!() }
//
//    fn report_missing_token(&self, recognizer: Parser) { unimplemented!() }
//
//    fn recover_inline(&self, recognizer: Parser) -> Token { unimplemented!() }
//
//    fn single_token_insertion(&self, recognizer: Parser) -> bool { unimplemented!() }
//
    fn single_token_deletion(&self, recognizer: &mut dyn Parser) -> Option<Box<dyn Token>> { unimplemented!() }
//
//    fn get_missing_symbol(&self, recognizer: Parser) -> Token { unimplemented!() }
//
//    fn get_expected_tokens(&self, recognizer: Parser) -> * IntervalSet { unimplemented!() }
//
//    fn get_token_error_display(&self, t: Token) -> String { unimplemented!() }
//
//    fn escape_wsand_quote(&self, s: String) -> String { unimplemented!() }
//
//    fn get_error_recovery_set(&self, recognizer: Parser) -> * IntervalSet { unimplemented!() }
//
//    fn consume_until(&self, recognizer: Parser, set: * IntervalSet) { unimplemented!() }
}

impl ErrorStrategy for DefaultErrorStrategy {
    fn reset(&mut self, recognizer: &mut Parser) {
        unimplemented!()
    }

    fn recover_inline(&mut self, recognizer: &mut Parser) -> Result<&dyn Token, ANTLRError> {
        unimplemented!()
    }

    fn recover(&mut self, recognizer: &mut Parser, e: &ANTLRError) {
        unimplemented!()
    }

    fn sync(&mut self, recognizer: &mut Parser) -> Result<(), ANTLRError> {
        return Ok(());
        //todo finish recovery
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
                if !self.single_token_deletion(recognizer).is_none() {
                    return Err(ANTLRError::InputMismatchError(InputMisMatchError::new()));
                }
            ATNSTATE_PLUS_LOOP_BACK |
            ATNSTATE_STAR_LOOP_BACK => {
                panic!("should not hit here yet")
            }
            _ => { panic!("invalid ANTState type id") }
        }

        Ok(())
    }

    fn in_error_recovery_mode(&mut self, recognizer: &mut Parser) -> bool {
        self.error_recovery_mode
    }

    fn report_error(&mut self, recognizer: &mut Parser, e: &ANTLRError) {
        unimplemented!()
    }

    fn report_match(&mut self, recognizer: &mut Parser) {
        println!("matched token succesfully {}", recognizer.get_input_stream().la(1))
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
 