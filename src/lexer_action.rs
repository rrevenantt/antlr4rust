use crate::lexer::Lexer;
use std::hash::Hash;

const lexer_action_type_channel: isize = 0;
const LexerActionTypeCustom: isize = 1;
const LexerActionTypeMode: isize = 2;
const LexerActionTypeMore: isize = 3;
const LexerActionTypePopMode: isize = 4;
const LexerActionTypePushMode: isize = 5;
const LexerActionTypeSkip: isize = 6;
const LexerActionTypeType: isize = 7;

pub enum LexerActionType {
    /**
     * The type of a {@link LexerChannelAction} action.
     */
    CHANNEL = 0,
    /**
     * The type of a {@link LexerCustomAction} action.
     */
    CUSTOM,
    /**
     * The type of a {@link LexerModeAction} action.
     */
    MODE,
    /**
     * The type of a {@link LexerMoreAction} action.
     */
    MORE,
    /**
     * The type of a {@link LexerPopModeAction} action.
     */
    POP_MODE,
    /**
     * The type of a {@link LexerPushModeAction} action.
     */
    PUSH_MODE,
    /**
     * The type of a {@link LexerSkipAction} action.
     */
    SKIP,
    /**
     * The type of a {@link LexerTypeAction} action.
     */
    TYPE,
}

pub trait LexerAction: Send + Sync {
    fn get_action_type(&self) -> isize;
    fn get_is_position_dependent(&self) -> bool;
    fn execute(&self, lexer: &Lexer);
}

#[derive(Eq, PartialEq, Hash)]
pub struct BaseLexerAction {
    action_type: isize,
    is_position_dependent: bool,
}

impl LexerAction for BaseLexerAction {
    fn get_action_type(&self) -> isize {
        unimplemented!()
    }

    fn get_is_position_dependent(&self) -> bool {
        unimplemented!()
    }

    fn execute(&self, lexer: &Lexer) {
        unimplemented!()
    }
}
//
//impl BaseLexerAction {
//    fn new_base_lexer_action(action isize) -> * BaseLexerAction { unimplemented!() }
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn get_action_type(&self) -> int { unimplemented!() }
//
//    fn get_is_position_dependent(&self) -> bool { unimplemented!() }
//
//    fn hash(&self) -> int { unimplemented!() }
//
//    fn equals(&self, other: LexerAction) -> bool { unimplemented!() }
//
//    pub struct LexerSkipAction {
//    base: BaseLexerAction,
//    }
//
//    fn new_lexer_skip_action() -> * LexerSkipAction { unimplemented!() }
//
//    var LexerSkipActionINSTANCE = NewLexerSkipAction()
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//    pub struct LexerTypeAction {
//    base: BaseLexerAction,
//    thetype: isize,
//    }
//
//    fn new_lexer_type_action(thetype isize) -> * LexerTypeAction { unimplemented!() }
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn hash(&self) -> int { unimplemented!() }
//
//    fn equals(&self, other: LexerAction) -> bool { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//    pub struct LexerPushModeAction {
//    base: BaseLexerAction,
//    mode: isize,
//    }
//
//    fn new_lexer_push_mode_action(mode isize) -> * LexerPushModeAction { unimplemented!() }
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn hash(&self) -> int { unimplemented!() }
//
//    fn equals(&self, other: LexerAction) -> bool { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//    pub struct LexerPopModeAction {
//    base: BaseLexerAction,
//    }
//
//    fn new_lexer_pop_mode_action() -> * LexerPopModeAction { unimplemented!() }
//
//    var LexerPopModeActionINSTANCE = NewLexerPopModeAction()
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//
//    pub struct LexerMoreAction {
//    base: BaseLexerAction,
//    }
//
//    fn new_lexer_more_action() -> * LexerMoreAction { unimplemented!() }
//
//    var LexerMoreActionINSTANCE = NewLexerMoreAction()
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//    pub struct LexerModeAction {
//    base: BaseLexerAction,
//    mode: isize,
//    }
//
//    fn new_lexer_mode_action(mode isize) -> * LexerModeAction { unimplemented!() }
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn hash(&self) -> int { unimplemented!() }
//
//    fn equals(&self, other: LexerAction) -> bool { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//
//    pub struct LexerCustomAction {
//    base: BaseLexerAction,
//    rule_index, actionIndex: int
//    }
//
//    fn new_lexer_custom_action(ruleIndex, actionIndex: isize) -> * LexerCustomAction { unimplemented!() }
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn hash(&self) -> int { unimplemented!() }
//
//    fn equals(&self, other: LexerAction) -> bool { unimplemented!() }
//
//    pub struct LexerChannelAction {
//    base: BaseLexerAction,
//    channel: isize,
//    }
//
//    fn new_lexer_channel_action(channel isize) -> * LexerChannelAction { unimplemented!() }
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn hash(&self) -> int { unimplemented!() }
//
//    fn equals(&self, other: LexerAction) -> bool { unimplemented!() }
//
//    fn String(&self) -> String { unimplemented!() }
//
//
//    pub struct LexerIndexedCustomAction {
//    base: BaseLexerAction,
//    offset: isize,
//    lexer_action: LexerAction,
//    is_position_dependent: bool,
//    }
//
//    fn new_lexer_indexed_custom_action(offset isize, lexerAction: LexerAction) -> * LexerIndexedCustomAction { unimplemented!() }
//
//    fn execute(&self, lexer: Lexer) { unimplemented!() }
//
//    fn hash(&self) -> int { unimplemented!() }
//
//    fn equals(&self, other: LexerAction) -> bool { unimplemented!() }
//}
//
