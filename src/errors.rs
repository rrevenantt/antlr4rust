use std::error::Error;
use std::fmt::{Debug, Display};
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::rc::Rc;

use crate::atn::ATN;
use crate::atn_simulator::IATNSimulator;
use crate::interval_set::IntervalSet;
use crate::parser::Parser;
use crate::parser_rule_context::ParserRuleContext;
use crate::token::{OwningToken, Token};
use crate::transition::PredicateTransition;
use crate::transition::TransitionType::TRANSITION_PREDICATE;

/// Main ANTLR4 Rust runtime error
#[derive(Debug)]
pub enum ANTLRError {
    LexerNoAltError { start_index: isize },
    NoAltError(NoViableAltError),
    InputMismatchError(InputMisMatchError),
    PredicateError(FailedPredicateError),
    IllegalStateError(String),
    /// Indicates that error should not be processed and parser should immediately return to caller
    FallThrough(Box<dyn Error>),
    /// Used to allow user to emit his own errors from parser actions or from custom error strategy
    OtherError(Box<dyn Error>),
}

impl Clone for ANTLRError {
    fn clone(&self) -> Self {
        match self {
            ANTLRError::LexerNoAltError { start_index } => ANTLRError::LexerNoAltError { start_index: *start_index },
            ANTLRError::NoAltError(e) => ANTLRError::NoAltError(e.clone()),
            ANTLRError::InputMismatchError(e) => ANTLRError::InputMismatchError(e.clone()),
            ANTLRError::PredicateError(e) => ANTLRError::PredicateError(e.clone()),
            ANTLRError::IllegalStateError(e) => ANTLRError::IllegalStateError(e.clone()),
            ANTLRError::FallThrough(_) => panic!("clone not supported"),
            ANTLRError::OtherError(_) => panic!("clone not supported"),
        }
    }
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
    fn get_expected_tokens(&self, _atn: &ATN) -> IntervalSet {
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
    ctx: Rc<dyn ParserRuleContext>
    //    input: Box<IntStream>,
}

impl BaseRecognitionError {
    pub fn get_expected_tokens(&self, recognizer: &dyn Parser) -> IntervalSet {
        recognizer.get_interpreter().atn()
            .get_expected_tokens(self.offending_state, &self.ctx)
    }

    fn new(recog: &mut dyn Parser) -> BaseRecognitionError {
        BaseRecognitionError {
            message: "".to_string(),
            offending_token: recog.get_current_token().to_owned(),
            offending_state: recog.get_state(),
            ctx: recog.get_parser_rule_context().clone()
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
//    ctx: Rc<dyn ParserRuleContext>,
    //    dead_end_configs: BaseATNConfigSet,
}

impl NoViableAltError {
    pub fn new(recog: &mut dyn Parser) -> NoViableAltError {
        Self {
            base: BaseRecognitionError {
                message: "".to_string(),
                offending_token: recog.get_current_token().to_owned(),
                offending_state: recog.get_state(),
                ctx: recog.get_parser_rule_context().clone(),
            },
            start_token: recog.get_current_token().to_owned(),
//            ctx: recog.get_parser_rule_context().clone()
        }
    }
    pub fn new_full(recog: &mut dyn Parser, start_token: OwningToken, offending_token: OwningToken) -> NoViableAltError {
        Self {
            base: BaseRecognitionError {
                message: "".to_string(),
                offending_token,
                offending_state: recog.get_state(),
                ctx: recog.get_parser_rule_context().clone(),
            },
            start_token,
//            ctx
        }
    }
}

//fn new_no_viable_alt_exception(recognizer: Parser, input: TokenStream, startToken: &Token, offendingToken: &Token, deadEndConfigs: ATNConfigSet, ctx: ParserRuleContext) -> NoViableAltError { unimplemented!() }

#[derive(Debug, Clone)]
pub struct InputMisMatchError {
    pub(crate) base: BaseRecognitionError,
}

impl InputMisMatchError {
    pub fn new(recognizer: &mut dyn Parser) -> InputMisMatchError {
//        println!("{:?}",Backtrace::new());
        InputMisMatchError {
            base: BaseRecognitionError::new(recognizer),
        }
    }

    pub fn with_state(recognizer: &mut dyn Parser, offending_state: isize, ctx: Rc<dyn ParserRuleContext>) -> InputMisMatchError {
        let mut a = Self::new(recognizer);
        a.base.ctx = ctx;
        a.base.offending_state = offending_state;
        a
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
            let pr = tr.deref().cast::<PredicateTransition>();
            (pr.rule_index, pr.pred_index)
        } else {
            (0, 0)
        };

        ANTLRError::PredicateError(FailedPredicateError {
            base: BaseRecognitionError {
                message: msg.unwrap_or_else(|| format!("failed predicate: {}", predicate.as_deref().unwrap_or("None"))),
                offending_token: recog.get_current_token().to_owned(),
                offending_state: recog.get_state(),
                ctx: recog.get_parser_rule_context().clone()
            },
            rule_index,
            predicate_index,
            predicate: predicate.unwrap_or_default(),
        })
    }
}
//fn new_failed_predicate_exception(recognizer: Parser, predicate: String, message: String) -> FailedPredicateError { unimplemented!() }
