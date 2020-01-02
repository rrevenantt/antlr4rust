use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fmt;
use std::fmt::Formatter;
use std::mem::discriminant;
use std::ops::Deref;

use crate::atn::ATN;
use crate::atn_config_set::ATNConfigSet;
use crate::atn_deserializer::cast;
use crate::atn_simulator::IATNSimulator;
use crate::char_stream::CharStream;
use crate::int_stream::IntStream;
use crate::interval_set::IntervalSet;
use crate::lexer::Lexer;
use crate::parser::{BaseParser, Parser};
use crate::token::{OwningToken, Token};
use crate::transition::PredicateTransition;
use crate::transition::TransitionType::TRANSITION_PREDICATE;

#[derive(Debug, Clone)]
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

impl RecognitionError for ANTLRError {
    fn get_offending_token(&self) -> Option<&dyn Token> {
        Some(match self {
            ANTLRError::NoAltError(e) => &e.base.offending_token,
            ANTLRError::InputMismatchError(e) => &e.base.offending_token,
            ANTLRError::PredicateError(e) => &e.base.offending_token,
            _ => return None
        })
    }
}

impl ANTLRError {
    fn get_expected_tokens(&self, atn: &ATN) -> IntervalSet {
//        atn.get_expected_tokens(se)
        unimplemented!()
    }
}

pub trait RecognitionError: Error {
    fn get_offending_token(&self) -> Option<&dyn Token>;
    fn get_message(&self) -> String { self.to_string() }
//    fn get_input_stream(&self) -> &IntStream;
}

#[derive(Debug, Clone)]
pub struct BaseRecognitionError {
    pub message: String,
    //    recognizer: Box<Recognizer>,
    pub offending_token: OwningToken,
    pub offending_state: isize,
    //    ctx: Box<RuleContext>,
    //    input: Box<IntStream>,
}

impl BaseRecognitionError {
    pub fn get_expected_tokens(&self, recognizer: &dyn Parser) -> IntervalSet {
        recognizer.get_interpreter().atn()
            .get_expected_tokens(self.offending_state, recognizer.get_parser_rule_context())
    }

    fn new(recog: &mut dyn Parser) -> BaseRecognitionError {
        BaseRecognitionError {
            message: "".to_string(),
            offending_token: recog.get_current_token().to_owned(),
            offending_state: recog.get_state(),
        }
    }
}

//impl RecognitionError for BaseRecognitionError {
//    fn get_offending_token(&self) -> &Token {
//        unimplemented!()
//    }
//
//    fn get_message(&self) -> String {
//        unimplemented!()
//    }
//
//    fn get_input_stream(&self) -> &IntStream {
//        unimplemented!()
//    }
//}


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct NoViableAltError {
    pub base: BaseRecognitionError,
    pub start_token: OwningToken,
    pub offending_token: OwningToken,
    //    ctx: ParserRuleContext,
    //    dead_end_configs: BaseATNConfigSet,
}

impl NoViableAltError {
    pub fn new() -> NoViableAltError {
        unimplemented!()
    }
}

//fn new_no_viable_alt_exception(recognizer: Parser, input: TokenStream, startToken: &Token, offendingToken: &Token, deadEndConfigs: ATNConfigSet, ctx: ParserRuleContext) -> NoViableAltError { unimplemented!() }

#[derive(Debug, Clone)]
pub struct InputMisMatchError {
    pub(crate) base: BaseRecognitionError,
}

impl InputMisMatchError {
    pub fn new(recognizer: &mut dyn Parser) -> InputMisMatchError {
        InputMisMatchError {
            base: BaseRecognitionError::new(recognizer)
        }
    }
}

//fn new_input_mis_match_exception(recognizer: Parser) -> InputMisMatchError { unimplemented!() }

#[derive(Debug, Clone)]
pub struct FailedPredicateError {
    pub(crate) base: BaseRecognitionError,
    rule_index: isize,
    predicate_index: isize,
    predicate: String,
}

impl FailedPredicateError {
    pub fn new(recog: &mut dyn Parser, predicate: Option<String>, msg: Option<String>) -> ANTLRError {
//        let predicate = predicate.map(|x|x.into());
//        let msg = msg.map(|x|x.into());

        let tr = recog.get_interpreter().atn()
            .states[recog.get_state() as usize]
            .get_transitions().first().unwrap();
        let (rule_index, predicate_index) = if tr.get_serialization_type() == TRANSITION_PREDICATE {
            let pr = unsafe { cast::<PredicateTransition>(tr.deref()) };
            (pr.rule_index, pr.pred_index)
        } else {
            (0, 0)
        };

        ANTLRError::PredicateError(FailedPredicateError {
            base: BaseRecognitionError {
                message: msg.unwrap_or_else(|| format!("failed predicate: {}", predicate.as_deref().unwrap_or("None"))),
                offending_token: recog.get_current_token().to_owned(),
                offending_state: recog.get_state(),
            },
            rule_index,
            predicate_index,
            predicate: predicate.unwrap_or_default(),
        })
    }
}
//fn new_failed_predicate_exception(recognizer: Parser, predicate: String, message: String) -> FailedPredicateError { unimplemented!() }
