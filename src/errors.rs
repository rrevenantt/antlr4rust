use std::error::Error;
use crate::token::Token;
use crate::int_stream::IntStream;

use crate::interval_set::IntervalSet;
use crate::lexer::Lexer;
use crate::char_stream::CharStream;
use crate::atn_config_set::ATNConfigSet;

use std::fmt::{Debug, Display};
use std::fmt::Formatter;
use std::fmt;
use std::mem::discriminant;

#[derive(Debug)]
pub enum ANTLRError {
    LexerNoAltError { start_index: isize },
    NoAltError(NoViableAltError),
    InputMismatchError(InputMisMatchError),
    PredicateError(FailedPredicateError),
    IllegalStateError(String),
}

impl Display for ANTLRError {
    fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
        <Self as Debug>::fmt(self, _f)
    }
}

impl Error for ANTLRError {}

pub trait RecognitionError {
    fn get_offending_token(&self) -> &Token;
    fn get_message(&self) -> String;
    fn get_input_stream(&self) -> &IntStream;
}

#[derive(Debug)]
pub struct BaseRecognitionError {
    message: String,
    //    recognizer: Box<Recognizer>,
    offending_token: Option<Box<Token>>,
    offending_state: isize,
    //    ctx: Box<RuleContext>,
    //    input: Box<IntStream>,
}

impl BaseRecognitionError {
    fn get_expected_tokens(&self) -> IntervalSet {
        unimplemented!()
    }

    fn new() -> BaseRecognitionError {
        unimplemented!()
    }
}

impl RecognitionError for BaseRecognitionError {
    fn get_offending_token(&self) -> &Token {
        unimplemented!()
    }

    fn get_message(&self) -> String {
        unimplemented!()
    }

    fn get_input_stream(&self) -> &IntStream {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct LexerNoViableAltError {
    base: BaseRecognitionError,
    start_index: isize,
    //    dead_end_configs: BaseATNConfigSet,
}
//
//fn new_lexer_no_viable_alt_exception(
//    _lexer: &dyn Lexer,
//    _input: &dyn CharStream,
//    _startIndex: isize,
////    _deadEndConfigs: &ATNConfigSet
//) -> LexerNoViableAltError {
//
//}

#[derive(Debug)]
pub struct NoViableAltError {
    base: BaseRecognitionError,
    start_token: Box<Token>,
    offending_token: Box<Token>,
    //    ctx: ParserRuleContext,
    //    dead_end_configs: BaseATNConfigSet,
}

impl NoViableAltError {
    pub fn new() -> NoViableAltError {
        unimplemented!()
    }
}

//fn new_no_viable_alt_exception(recognizer: Parser, input: TokenStream, startToken: &Token, offendingToken: &Token, deadEndConfigs: ATNConfigSet, ctx: ParserRuleContext) -> NoViableAltError { unimplemented!() }

#[derive(Debug)]
pub struct InputMisMatchError {
    base: BaseRecognitionError,
}

impl InputMisMatchError {
    pub fn new() -> InputMisMatchError {
        InputMisMatchError {
            base: BaseRecognitionError {
                message: "".to_string(),
                offending_token: None,
                offending_state: 0,
            }
        }
    }
}

//fn new_input_mis_match_exception(recognizer: Parser) -> InputMisMatchError { unimplemented!() }

#[derive(Debug)]
pub struct FailedPredicateError {
    base: BaseRecognitionError,
    rule_index: isize,
    predicate_index: isize,
    predicate: String,
}

//fn new_failed_predicate_exception(recognizer: Parser, predicate: String, message: String) -> FailedPredicateError { unimplemented!() }
