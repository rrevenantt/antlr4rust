use std::hash::Hash;

use crate::lexer::Lexer;

pub(crate) const LEXER_ACTION_TYPE_CHANNEL: isize = 0;
pub(crate) const LEXER_ACTION_TYPE_CUSTOM: isize = 1;
pub(crate) const LEXER_ACTION_TYPE_MODE: isize = 2;
pub(crate) const LEXER_ACTION_TYPE_MORE: isize = 3;
pub(crate) const LEXER_ACTION_TYPE_POP_MODE: isize = 4;
pub(crate) const LEXER_ACTION_TYPE_PUSH_MODE: isize = 5;
pub(crate) const LEXER_ACTION_TYPE_SKIP: isize = 6;
pub(crate) const LEXER_ACTION_TYPE_TYPE: isize = 7;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) enum LexerAction {
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
    //    fn get_action_type(&self) -> isize {
    //        unimplemented!()
    ////        unsafe {discriminant_value(self)} as isize
    //    }
    pub fn is_position_dependent(&self) -> bool {
        match self {
            LexerAction::LexerCustomAction { .. }
            | LexerAction::LexerIndexedCustomAction { .. } => true,
            _ => false,
        }
    }
    pub(crate) fn execute<'input, T: Lexer<'input>>(&self, lexer: &mut T) {
        match self {
            &LexerAction::LexerChannelAction(channel) => lexer.set_channel(channel),
            &LexerAction::LexerCustomAction {
                rule_index,
                action_index,
            } => {
                lexer.action(None, rule_index, action_index);
            }
            &LexerAction::LexerModeAction(mode) => lexer.set_mode(mode as usize),
            &LexerAction::LexerMoreAction => lexer.more(),
            &LexerAction::LexerPopModeAction => {
                lexer.pop_mode();
            }
            &LexerAction::LexerPushModeAction(mode) => lexer.push_mode(mode as usize),
            &LexerAction::LexerSkipAction => lexer.skip(),
            &LexerAction::LexerTypeAction(ty) => lexer.set_type(ty),
            &LexerAction::LexerIndexedCustomAction { ref action, .. } => action.execute(lexer),
        }
    }
}
