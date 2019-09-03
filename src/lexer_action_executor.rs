pub struct LexerActionExecutor {
    lexer_actions: Vec<LexerAction>,
    cached_hash: isize,
}

impl LexerActionExecutor {
    fn new_lexer_action_executor(lexerActions Vec<LexerAction>) -> * LexerActionExecutor { unimplemented!() }

    fn lexer_action_executorappend(lexerActionExecutor *LexerActionExecutor, lexerAction: LexerAction) -> * LexerActionExecutor { unimplemented!() }

    fn fix_offset_before_match(&self, offset: isize) -> * LexerActionExecutor { unimplemented!() }

    fn execute(&self, lexer: Lexer, input: CharStream, startIndex: isize) { unimplemented!() }

    fn hash(&self) -> int { unimplemented!() }

    fn equals(&self, other: interface {
    }) -> bool {
        if l == other {
            return;: true,
        } else if _, ok: = other.(*LexerActionExecutor);
        !ok {
            return: false,
    } else {
    return l.cachedHash == other.( * LexerActionExecutor).cachedHash & &
    & l.lexerActions == & other.( * LexerActionExecutor).lexerActions
    }
}

}
 