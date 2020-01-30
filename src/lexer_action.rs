use std::hash::Hash;

use crate::lexer::{BaseLexer, Lexer, LexerRecog};
use crate::parser_rule_context::empty_ctx;
use crate::recognizer::Recognizer;

//use std::mem::discriminant;
//use std::intrinsics::discriminant_value;

pub const LEXER_ACTION_TYPE_CHANNEL: isize = 0;
pub const LEXER_ACTION_TYPE_CUSTOM: isize = 1;
pub const LEXER_ACTION_TYPE_MODE: isize = 2;
pub const LEXER_ACTION_TYPE_MORE: isize = 3;
pub const LEXER_ACTION_TYPE_POP_MODE: isize = 4;
pub const LEXER_ACTION_TYPE_PUSH_MODE: isize = 5;
pub const LEXER_ACTION_TYPE_SKIP: isize = 6;
pub const LEXER_ACTION_TYPE_TYPE: isize = 7;

//pub enum LexerActionType {
//    /**
//     * The type of a {@link LexerChannelAction} action.
//     */
//    CHANNEL = 0,
//    /**
//     * The type of a {@link LexerCustomAction} action.
//     */
//    CUSTOM,
//    /**
//     * The type of a {@link LexerModeAction} action.
//     */
//    MODE,
//    /**
//     * The type of a {@link LexerMoreAction} action.
//     */
//    MORE,
//    /**
//     * The type of a {@link LexerPopModeAction} action.
//     */
//    POP_MODE,
//    /**
//     * The type of a {@link LexerPushModeAction} action.
//     */
//    PUSH_MODE,
//    /**
//     * The type of a {@link LexerSkipAction} action.
//     */
//    SKIP,
//    /**
//     * The type of a {@link LexerTypeAction} action.
//     */
//    TYPE,
//}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LexerAction {
    LexerChannelAction(isize),
    LexerCustomAction {
        rule_index: isize,
        action_index: isize,
    },
    LexerModeAction(isize),
    LexerMoreAction,
    LexerPopModeAction,
    LexerPushModeAction(isize),
    LexerSkipAction,
    LexerTypeAction(isize),
    LexerIndexedCustomAction {
        offset: isize,
        action: Box<LexerAction>,
    },
}

impl LexerAction {
    fn get_action_type(&self) -> isize {
        unimplemented!()
//        unsafe {discriminant_value(self)} as isize
    }
    pub fn is_position_dependent(&self) -> bool {
        match self {
            LexerAction::LexerCustomAction { .. } |
            LexerAction::LexerIndexedCustomAction { .. } => true,
            _ => false
        }
    }
    pub(crate) fn execute(&self, lexer: &mut dyn Lexer) {
        match self {
            &LexerAction::LexerChannelAction(channel) => lexer.set_channel(channel),
            &LexerAction::LexerCustomAction { rule_index, action_index } => {
                lexer.action(&*empty_ctx(), rule_index, action_index);
            },
            &LexerAction::LexerModeAction(mode) => lexer.set_mode(mode as usize),
            &LexerAction::LexerMoreAction => lexer.more(),
            &LexerAction::LexerPopModeAction => { lexer.pop_mode(); },
            &LexerAction::LexerPushModeAction(mode) => lexer.push_mode(mode as usize),
            &LexerAction::LexerSkipAction => lexer.skip(),
            &LexerAction::LexerTypeAction(ty) => lexer.set_type(ty),
            &LexerAction::LexerIndexedCustomAction { ref action, .. } => action.execute(lexer),
        }
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
