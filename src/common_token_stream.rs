pub struct CommonTokenStream {
    channel: isize,

    fetched_eof: bool,

    index: isize,

    token_source: TokenSource,

    tokens: Vec<Token>,
}

impl CommonTokenStream {
    fn new_common_token_stream(lexer: Lexer, channel: isize) -> CommonTokenStream { unimplemented!() }

    fn get_all_tokens(&self) -> Vec<Token> { unimplemented!() }

    fn mark(&self) -> int { unimplemented!() }

    fn release(&self, marker: isize) { unimplemented!() }

    fn reset(&self) { unimplemented!() }

    fn seek(&self, index: isize) { unimplemented!() }

    fn get(&self, index: isize) -> Token { unimplemented!() }

    fn consume(&self) { unimplemented!() }

    fn sync(&self, i: isize) -> bool { unimplemented!() }

    fn fetch(&self, n: isize) -> int { unimplemented!() }

    fn get_tokens(&self, start: isize, stop: isize, types: &IntervalSet) -> Vec<Token> { unimplemented!() }

    fn la(&self, i: isize) -> int { unimplemented!() }

    fn lazy_init(&self) { unimplemented!() }

    fn setup(&self) { unimplemented!() }

    fn get_token_source(&self) -> TokenSource { unimplemented!() }

    fn set_token_source(&self, tokenSource: TokenSource) { unimplemented!() }

    fn next_token_on_channel(&self, i: isize, channel: isize) -> int { unimplemented!() }

    fn previous_token_on_channel(&self, i: isize, channel: isize) -> int { unimplemented!() }

    fn get_hidden_tokens_to_right(&self, tokenIndex: isize, channel: isize) -> Vec<Token> { unimplemented!() }

    fn get_hidden_tokens_to_left(&self, tokenIndex: isize, channel: isize) -> Vec<Token> { unimplemented!() }

    fn filter_for_channel(&self, left: isize, right: isize, channel: isize) -> Vec<Token> { unimplemented!() }

    fn get_source_name(&self) -> String { unimplemented!() }

    fn size(&self) -> int { unimplemented!() }

    fn index(&self) -> int { unimplemented!() }

    fn get_all_text(&self) -> String { unimplemented!() }

    fn get_text_from_tokens(&self, start: Token, end: Token) -> String { unimplemented!() }

    fn get_text_from_rule_context(&self, interval: RuleContext) -> String { unimplemented!() }

    fn get_text_from_interval(&self, interval: &Interval) -> String { unimplemented!() }

    fn fill(&self) { unimplemented!() }

    fn adjust_seek_index(&self, i: isize) -> int { unimplemented!() }

    fn lb(&self, k: isize) -> Token { unimplemented!() }

    fn lt(&self, k: isize) -> Token { unimplemented!() }

    fn get_number_of_on_channel_tokens(&self) -> int { unimplemented!() }
}
 