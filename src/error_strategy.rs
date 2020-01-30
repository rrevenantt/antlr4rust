use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use crate::atn_simulator::IATNSimulator;
use crate::atn_state::*;
use crate::dfa::ScopeExt;
use crate::errors::{ANTLRError, FailedPredicateError, InputMisMatchError, NoViableAltError, RecognitionError};
use crate::interval_set::IntervalSet;
use crate::parser::Parser;
use crate::parser_rule_context::ParserRuleContext;
use crate::token::{OwningToken, Token, TOKEN_DEFAULT_CHANNEL, TOKEN_EOF, TOKEN_EPSILON, TOKEN_INVALID_TYPE};
use crate::transition::RuleTransition;
use crate::utils::escape_whitespaces;

/// The interface for defining strategies to deal with syntax errors encountered
/// during a parse by ANTLR-generated parsers. We distinguish between three
/// different kinds of errors:
/// <ul>
/// <li>The parser could not figure out which path to take in the ATN (none of
/// the available alternatives could possibly match)</li>
/// <li>The current input does not match what we were looking for</li>
/// <li>A predicate evaluated to false</li>
/// </ul>
///
/// Implementations of this interface report syntax errors by calling [`Parser.notifyErrorListeners`]
///
/// [`Parser.notifyErrorListeners`]: todo
pub trait ErrorStrategy {
    fn reset(&mut self, recognizer: &mut dyn Parser);
    fn recover_inline(&mut self, recognizer: &mut dyn Parser) -> Result<OwningToken, ANTLRError>;
    fn recover(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError) -> Result<(), ANTLRError>;
    fn sync(&mut self, recognizer: &mut dyn Parser) -> Result<(), ANTLRError>;
    fn in_error_recovery_mode(&mut self, recognizer: &mut dyn Parser) -> bool;
    fn report_error(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError);
    fn report_match(&mut self, recognizer: &mut dyn Parser);
}

pub struct DefaultErrorStrategy {
    error_recovery_mode: bool,
    last_error_index: isize,
    last_error_states: Option<IntervalSet>,
    next_tokens_state: isize,
    next_tokens_ctx: Option<Rc<dyn ParserRuleContext>>,
}


impl DefaultErrorStrategy {
    pub fn new() -> DefaultErrorStrategy {
        DefaultErrorStrategy {
            error_recovery_mode: false,
            last_error_index: -1,
            last_error_states: None,
            next_tokens_state: ATNSTATE_INVALID_STATE_NUMBER,
            next_tokens_ctx: None,
        }
    }

    fn begin_error_condition(&mut self, _recognizer: &dyn Parser) {
        self.error_recovery_mode = true;
    }

    fn end_error_condition(&mut self, _recognizer: &dyn Parser) {
        self.error_recovery_mode = false;
        self.last_error_index = -1;
        self.last_error_states = None;
    }

    fn report_no_viable_alternative(&self, recognizer: &mut dyn Parser, e: &NoViableAltError) -> String {
        let input = if e.start_token.token_type == TOKEN_EOF {
            "<EOF>".to_owned()
        } else {
            recognizer.get_input_stream_mut().get_text_from_tokens(&e.start_token, &e.base.offending_token)
        };

        format!("no viable alternative at input '{}'", input)
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
        let _token_name = self.get_token_error_display(t);
        let msg = format!("missing {} at {}",
                          expecting,
                          self.get_token_error_display(t)
        );
        let t = t.get_token_index();
        recognizer.notify_error_listeners(msg, Some(t), None);
    }

    fn single_token_insertion(&mut self, recognizer: &mut dyn Parser) -> bool {
        let current_token = recognizer.get_input_stream_mut().la(1);

        let atn = recognizer.get_interpreter().atn();
        let current_state = atn.states[recognizer.get_state() as usize].as_ref();
        let next = current_state.get_transitions().first().unwrap().get_target();
        let expect_at_ll2 = atn.next_tokens_in_ctx(
            atn.states[next].as_ref(),
            Some(recognizer.get_parser_rule_context().deref()),
        );
        if expect_at_ll2.contains(current_token) {
            self.report_missing_token(recognizer);
            return true;
        }
        false
    }

    fn single_token_deletion<'a>(&mut self, recognizer: &'a mut dyn Parser) -> Option<&'a dyn Token> {
        let next_token_type = recognizer.get_input_stream_mut().la(2);
        let expecting = self.get_expected_tokens(recognizer);
//        println!("expecting {}", expecting.to_token_string(recognizer.get_vocabulary()));
        if expecting.contains(next_token_type) {
            self.report_unwanted_token(recognizer);
            recognizer.consume(self);
            self.report_match(recognizer);
            let matched_symbol = recognizer.get_current_token();
            return Some(matched_symbol);
        }
        None
    }

    fn get_missing_symbol(&self, recognizer: &mut dyn Parser) -> OwningToken {
        let expected = self.get_expected_tokens(recognizer);
        let expected_token_type = expected.get_min().unwrap_or(TOKEN_INVALID_TYPE);
        let token_text = if expected_token_type == TOKEN_EOF {
            "<missing EOF>".to_owned()
        } else {
            format!("<missing {}>", recognizer.get_vocabulary().get_display_name(expected_token_type))
        };
//        let look_back =
        let mut curr = recognizer.get_current_token();
        if curr.get_token_type() == TOKEN_EOF {
            curr = recognizer.get_input_stream().run(|it| it.get((it.index() - 1).max(0)));
        }
        let (line, column) = (curr.get_line(), curr.get_column());
        recognizer.get_token_factory()
            .create(
                None,
                expected_token_type,
                TOKEN_DEFAULT_CHANNEL,
                -1,
                -1,
                line,
                column,
            ).to_owned()
            .modify_with(|it| it.text = token_text)
    }

    fn get_expected_tokens(&self, recognizer: &dyn Parser) -> IntervalSet {
        recognizer.get_expected_tokens()
    }

    fn get_token_error_display(&self, t: &dyn Token) -> String {
        let text = t.get_text();
        self.escape_wsand_quote(text)
    }

    fn escape_wsand_quote(&self, s: &str) -> String {
        format!("'{}'", escape_whitespaces(s, false))
    }

    fn get_error_recovery_set(&self, recognizer: &dyn Parser) -> IntervalSet {
        let atn = recognizer.get_interpreter().atn();
        let mut ctx = Some(recognizer.get_parser_rule_context().clone());
        let mut recover_set = IntervalSet::new();
        while let Some(c) = ctx {
            if c.get_invoking_state() < 0 { break; }

            let invoking_state = atn.states[c.get_invoking_state() as usize].as_ref();
            let tr = invoking_state.get_transitions().first().unwrap().as_ref();
            let tr = tr.cast::<RuleTransition>();
            let follow = atn.next_tokens(atn.states[tr.follow_state].as_ref());
            recover_set.add_set(follow);
            ctx = c.peek_parent();
        }
        recover_set.remove_one(TOKEN_EPSILON);
        return recover_set;
    }

    fn consume_until(&mut self, recognizer: &mut dyn Parser, set: &IntervalSet) {
        let mut ttype = recognizer.get_input_stream_mut().la(1);
        while ttype != TOKEN_EOF && !set.contains(ttype) {
            recognizer.consume(self);
            ttype = recognizer.get_input_stream_mut().la(1);
        }
    }
}

impl ErrorStrategy for DefaultErrorStrategy {
    fn reset(&mut self, _recognizer: &mut dyn Parser) {
        unimplemented!()
    }

    fn recover_inline(&mut self, recognizer: &mut dyn Parser) -> Result<OwningToken, ANTLRError> {
        let t = self.single_token_deletion(recognizer).map(|it| it.to_owned());
        if let Some(t) = t {
            recognizer.consume(self);
            return Ok(t);
        }

        if self.single_token_insertion(recognizer) {
            return Ok(self.get_missing_symbol(recognizer));
        }

        if let Some(next_tokens_state) = &self.next_tokens_ctx {
            Err(ANTLRError::InputMismatchError(InputMisMatchError::with_state(recognizer, self.next_tokens_state, next_tokens_state.clone())))
        } else {
            Err(ANTLRError::InputMismatchError(InputMisMatchError::new(recognizer)))
        }
//        Err(ANTLRError::IllegalStateError("aaa".to_string()))
    }

    fn recover(&mut self, recognizer: &mut dyn Parser, _e: &ANTLRError) -> Result<(), ANTLRError> {
        if self.last_error_index == recognizer.get_input_stream_mut().index()
            && self.last_error_states.is_some()
            && self.last_error_states.as_ref().unwrap().contains(recognizer.get_state()) {
            recognizer.consume(self)
        }

        self.last_error_index = recognizer.get_input_stream_mut().index();
        self.last_error_states.get_or_insert(IntervalSet::new()).apply(
            |x| x.add_one(recognizer.get_state())
        );
        let follow_set = self.get_error_recovery_set(recognizer);
        self.consume_until(recognizer, &follow_set);
        Ok(())
    }

    fn sync(&mut self, recognizer: &mut dyn Parser) -> Result<(), ANTLRError> {
        if self.in_error_recovery_mode(recognizer) { return Ok(()); }
        let next = recognizer.get_input_stream_mut().la(1);
        let state = recognizer.get_interpreter().atn().states[recognizer.get_state() as usize].as_ref();

        let next_tokens = recognizer.get_interpreter().atn().next_tokens(state);
//        println!("{:?}",next_tokens);


        if next_tokens.contains(next) {
            self.next_tokens_state = ATNSTATE_INVALID_STATE_NUMBER;
            self.next_tokens_ctx = None;
            return Ok(());
        }

        if next_tokens.contains(TOKEN_EPSILON) {
            if self.next_tokens_ctx.is_none() {
                self.next_tokens_state = recognizer.get_state();
                self.next_tokens_ctx = Some(recognizer.get_parser_rule_context().clone());
            }
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

    fn in_error_recovery_mode(&mut self, _recognizer: &mut dyn Parser) -> bool {
        self.error_recovery_mode
    }

    fn report_error(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError) {
        if self.in_error_recovery_mode(recognizer) { return; }

        self.begin_error_condition(recognizer);
        let msg = match e {
            ANTLRError::NoAltError(e) => self.report_no_viable_alternative(recognizer, e),
            ANTLRError::InputMismatchError(e) => self.report_input_mismatch(recognizer, e),
            ANTLRError::PredicateError(e) => self.report_failed_predicate(recognizer, e),
            _ => e.to_string()
        };
        let offending_token_index = e.get_offending_token().map(|it| it.get_token_index());
        recognizer.notify_error_listeners(msg, offending_token_index, Some(&e))
    }

    fn report_match(&mut self, recognizer: &mut dyn Parser) {
        self.end_error_condition(recognizer);
        //println!("matched token succesfully {}", recognizer.get_input_stream().la(1))
    }
}

/**
This implementation of `ANTLRErrorStrategy` responds to syntax errors
by immediately canceling the parse operation with a
`ParseCancellationException`. The implementation ensures that the
[`ParserRuleContext.exception`] field is set for all parse tree nodes
that were not completed prior to encountering the error.

<p> This error strategy is useful in the following scenarios.</p>

<ul>
<li><strong>Two-stage parsing:</strong> This error strategy allows the first
stage of two-stage parsing to immediately terminate if an error is
encountered, and immediately fall back to the second stage. In addition to
avoiding wasted work by attempting to recover from errors here, the empty
implementation of `sync` improves the performance of
the first stage.</li>
<li><strong>Silent validation:</strong> When syntax errors are not being
reported or logged, and the parse result is simply ignored if errors occur,
the `BailErrorStrategy` avoids wasting work on recovering from errors
when the result will be ignored either way.</li>
</ul>

# Usage
```ignore
use antlr_rust::error_strategy::BailErrorStrategy;
myparser.err_handler = BailErrorStrategy::new();
```

[`ParserRuleContext.exception`]: todo
*/
pub struct BailErrorStrategy(DefaultErrorStrategy);

impl BailErrorStrategy {
    pub fn new() -> BailErrorStrategy { Self(DefaultErrorStrategy::new()) }

    fn process_error(&self, recognizer: &mut dyn Parser, e: &ANTLRError) -> ANTLRError {
        let mut ctx = recognizer.get_parser_rule_context().clone();
        let _: Option<()> = try {
            loop {
                ctx.set_exception(e.clone());
                ctx = ctx.get_parent()?
            }
        };
        return ANTLRError::FallThrough(Box::new(ParseCancelledError(e.clone())));
    }
}

#[derive(Debug)]
pub struct ParseCancelledError(ANTLRError);

impl Error for ParseCancelledError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl Display for ParseCancelledError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("ParseCancelledError, caused by ");
        self.0.fmt(f);
        Ok(())
    }
}

impl ErrorStrategy for BailErrorStrategy {
    fn reset(&mut self, recognizer: &mut dyn Parser) { self.0.reset(recognizer) }

    fn recover_inline(&mut self, recognizer: &mut dyn Parser) -> Result<OwningToken, ANTLRError> {
        let err = ANTLRError::InputMismatchError(InputMisMatchError::new(recognizer));

        Err(self.process_error(recognizer, &err))
    }

    fn recover(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError) -> Result<(), ANTLRError> {
        Err(self.process_error(recognizer, &e))
    }

    fn sync(&mut self, _recognizer: &mut dyn Parser) -> Result<(), ANTLRError> { /* empty */ Ok(()) }

    fn in_error_recovery_mode(&mut self, recognizer: &mut dyn Parser) -> bool { self.0.in_error_recovery_mode(recognizer) }

    fn report_error(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError) { self.0.report_error(recognizer, e) }

    fn report_match(&mut self, recognizer: &mut dyn Parser) { self.0.report_match(recognizer) }
}
 