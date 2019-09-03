use crate::semantic_context::SemanticContext;
use crate::atn_state::{ATNState, ATNStateRef, ATNStateType};

use crate::prediction_context::PredictionContext;
use crate::atn_state::ATNBlockStart::BasicBlockStart;
use std::hash::{Hash, Hasher};
use murmur3::murmur3_32::MurmurHasher;
use crate::atn_config::ATNConfigType::LexerATNConfig;
use crate::atn_state::ATNStateType::DecisionState;

pub trait ATNConfig: Sync + Send {
    fn get_state(&self) -> ATNStateRef;
    fn get_alt(&self) -> isize;
    fn get_type(&self) -> &ATNConfigType;
    //    fn get_semantic_context(&self) -> &SemanticContext;

    fn get_context(&self) -> Option<&PredictionContext>;
    fn take_context(&mut self) -> PredictionContext;
    fn set_context(&mut self, v: Box<PredictionContext>);

    fn get_reaches_into_outer_context(&self) -> isize;
    fn set_reaches_into_outer_context(&mut self, v: isize);

    fn get_precedence_filter_suppressed(&self) -> bool;
    fn set_precedence_filter_suppressed(&mut self, v: bool);
}

impl Eq for dyn ATNConfig {}

impl PartialEq for dyn ATNConfig {
    fn eq(&self, other: &Self) -> bool {
        self.get_state() == other.get_state() && self.get_alt() == other.get_alt()
            && self.get_context() == other.get_context()
        // && semantic context
    }
}

impl Hash for dyn ATNConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.get_state() as i32);
        state.write_i32(self.get_alt() as i32);
        match self.get_context() {
            None => state.write_i32(0),
            Some(c) => c.hash(state),
        }
        //todo semantic context
    }
}

#[derive(Clone)]
pub struct BaseATNConfig {
    precedence_filter_suppressed: bool,
    state: ATNStateRef,
    alt: isize,
    context: Option<Box<PredictionContext>>,
    //    semantic_context: Box<SemanticContext>,
    reaches_into_outer_context: isize,
    pub config_type: ATNConfigType,
}

#[derive(Eq, PartialEq, Clone)]
pub enum ATNConfigType {
    BaseATNConfig,
    LexerATNConfig {
        //    lexer_action_executor:  LexerActionExecutor,
        passed_through_non_greedy_decision: bool,
    },
}

impl BaseATNConfig {
    fn new_base_atnconfig7(_old: &BaseATNConfig) -> BaseATNConfig {
        unimplemented!()
    }

    fn new_base_atnconfig6(
        state: ATNStateRef,
        alt: isize,
        context: Option<PredictionContext>,
    ) -> BaseATNConfig {
        BaseATNConfig {
            precedence_filter_suppressed: false,
            state,
            alt,
            context: context.map(|x| Box::new(x)),
            reaches_into_outer_context: 0,
            config_type: ATNConfigType::BaseATNConfig,
        }
    }

    fn new_base_atnconfig5(
        _state: &ATNState,
        _alt: isize,
        _context: &PredictionContext,
        _semanticContext: &SemanticContext,
    ) -> BaseATNConfig {
        unimplemented!()
    }

    fn new_base_atnconfig4(_c: &ATNConfig, _state: &ATNState) -> BaseATNConfig {
        unimplemented!()
    }

    fn new_base_atnconfig3(
        _c: &ATNConfig,
        _state: &ATNState,
        _semanticContext: &SemanticContext,
    ) -> BaseATNConfig {
        unimplemented!()
    }

    fn new_base_atnconfig2(_c: &ATNConfig, _semanticContext: &SemanticContext) -> BaseATNConfig {
        unimplemented!()
    }

    fn new_base_atnconfig1(
        _c: &ATNConfig,
        _state: &ATNState,
        _context: &PredictionContext,
    ) -> BaseATNConfig {
        unimplemented!()
    }

    fn new_base_atnconfig(
        _c: &ATNConfig,
        _state: &ATNState,
        _context: &PredictionContext,
        _semanticContext: &SemanticContext,
    ) -> BaseATNConfig {
        unimplemented!()
    }

    pub fn new_lexer_atnconfig6(
        _state: ATNStateRef,
        _alt: isize,
        _context: PredictionContext,
    ) -> BaseATNConfig {
        let mut atnconfig = BaseATNConfig::new_base_atnconfig6(_state, _alt, Some(_context));
        atnconfig.config_type = ATNConfigType::LexerATNConfig {
            passed_through_non_greedy_decision: false,
        };
        atnconfig
    }

    //fn new_lexer_atnconfig5(state: &ATNState, alt: isize, context: PredictionContext, lexerActionExecutor: LexerActionExecutor) ->  LexerATNConfig { unimplemented!() }

    fn new_lexer_atnconfig4(_c: &ATNConfig, _state: &ATNState) -> BaseATNConfig {
        unimplemented!()
    }

    pub(crate) fn new_lexer_atnconfig3(
        c: &ATNConfig,
        state: &ATNState, /*, lexerActionExecutor:  LexerActionExecutor*/
    ) -> BaseATNConfig {
        let prediction_context = c.get_context().map(|x| x.clone());
        Self::new_lexer_atnconfig2(c, state, prediction_context)
    }

    pub(crate) fn new_lexer_atnconfig2(
        c: &ATNConfig,
        state: &ATNState,
        _context: Option<PredictionContext>,
    ) -> BaseATNConfig {
        let mut r =
            Self::new_base_atnconfig6(state.get_state_number(), c.get_alt(), _context);
        r.config_type = ATNConfigType::LexerATNConfig {
            passed_through_non_greedy_decision: check_non_greedy_decision(c, state),
        };
        r
    }

    fn new_lexer_atnconfig1(
        _state: &ATNState,
        _alt: isize,
        _context: &PredictionContext,
    ) -> BaseATNConfig {
        unimplemented!()
    }

    //fn check_non_greedy_decision(source LexerATNConfig, target: &ATNState) -> bool { unimplemented!() }
}

fn check_non_greedy_decision(source: &ATNConfig, target: &ATNState) -> bool {
    if let LexerATNConfig {
        passed_through_non_greedy_decision: true,
    } = source.get_type()
    {
        return true;
    }
    if let ATNStateType::DecisionState {
        nongreedy: true, ..
    } = target.get_state_type()
    {
        return true;
    }
    false
}

impl ATNConfig for BaseATNConfig {
    fn get_state(&self) -> ATNStateRef {
        self.state
    }

    fn get_alt(&self) -> isize {
        self.alt
    }

    fn get_type(&self) -> &ATNConfigType {
        &self.config_type
    }

    //    fn get_semantic_context(&self) -> &SemanticContext { unimplemented!() }

    fn get_context(&self) -> Option<&PredictionContext> {
        self.context.as_deref()
    }

    fn take_context(&mut self) -> PredictionContext {
        *self.context.take().unwrap()
    }

    fn set_context(&mut self, _v: Box<PredictionContext>) {
        self.context = Some(_v);
    }

    fn get_reaches_into_outer_context(&self) -> isize {
        self.reaches_into_outer_context
    }

    fn set_reaches_into_outer_context(&mut self, _v: isize) {
        self.reaches_into_outer_context = _v
    }

    fn get_precedence_filter_suppressed(&self) -> bool {
        self.precedence_filter_suppressed
    }

    fn set_precedence_filter_suppressed(&mut self, _v: bool) {
        self.precedence_filter_suppressed = _v;
    }
}
