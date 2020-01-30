use std::ops::Deref;

use crate::errors::ANTLRError;
use crate::int_stream::{EOF, IntStream, IterWrapper};
use crate::token::{Token, TOKEN_DEFAULT_CHANNEL, TOKEN_INVALID_TYPE};
use crate::token_source::TokenSource;
use crate::token_stream::{TokenStream, UnbufferedTokenStream};

pub struct CommonTokenStream<T: TokenSource> {
    base: UnbufferedTokenStream<T>,
    channel: isize,
}

impl<T: TokenSource> IntStream for CommonTokenStream<T> {
    fn consume(&mut self) -> Result<(), ANTLRError> {
        self.base.consume()?;
//        self.base.p = self.next_token_on_channel(self.base.p,self.channel);
//        self.base.current_token_index = self.base.p;
        let next = self.next_token_on_channel(self.base.p, self.channel, 1);
        self.base.seek(next);
        Ok(())
    }

    fn la(&mut self, i: isize) -> isize {
        self.lt(i).map(Token::get_token_type).unwrap_or(TOKEN_INVALID_TYPE)
    }

    fn mark(&mut self) -> isize {
        0
    }

    fn release(&mut self, _marker: isize) {}

    fn index(&self) -> isize {
        self.base.index()
    }

    fn seek(&mut self, index: isize) {
        self.base.seek(index);
    }

    #[inline(always)]
    fn size(&self) -> isize {
        self.base.size()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }
}

impl<T: TokenSource> TokenStream for CommonTokenStream<T> {
    fn lt(&mut self, k: isize) -> Option<&dyn Token> {
        if k == 0 { panic!(); }
        if k < 0 { return self.lb(-k); }
        let mut i = self.base.p;
        let mut n = 1; // we know tokens[p] is a good one
        // find k good tokens
        while n < k {
            // skip off-channel tokens, but make sure to not look past EOF
            if self.sync(i + 1) {
                i = self.next_token_on_channel(i + 1, self.channel, 1);
            }
            n += 1;
        }
//		if ( i>range ) range = i;
        return self.base.tokens.get(i as usize).map(Deref::deref)
    }

    fn get(&self, index: isize) -> &dyn Token {
        self.base.get(index)
    }

    fn get_token_source(&self) -> &dyn TokenSource {
        self.base.get_token_source()
    }

    fn get_all_text(&self) -> String {
        self.get_text_from_interval(0, self.size() - 1)
    }

    fn get_text_from_interval(&self, start: isize, stop: isize) -> String {
        self.base.get_text_from_interval(start, stop)
    }

    fn get_text_from_tokens(&self, a: &dyn Token, b: &dyn Token) -> String {
        self.base.get_text_from_tokens(a, b)
    }
}

impl<T: TokenSource> CommonTokenStream<T> {
    pub fn new(lexer: T) -> CommonTokenStream<T> {
        Self::with_channel(lexer, TOKEN_DEFAULT_CHANNEL)
    }

    pub fn with_channel(lexer: T, channel: isize) -> CommonTokenStream<T> {
        let mut r = CommonTokenStream {
            base: UnbufferedTokenStream::new_buffered(lexer),
            channel,
        };
        r.sync(0);
        r
    }
//
//    fn get_all_tokens(&self) -> Vec<Token> { unimplemented!() }
//
//    fn reset(&self) { unimplemented!() }

    pub fn iter(&mut self) -> IterWrapper<Self> {
        IterWrapper(self)
    }


    fn sync(&mut self, i: isize) -> bool {
        let need = i - self.size() + 1;
        if need > 0 {
            let fetched = self.base.fill(need);
            return fetched >= need;
        }

        true
    }
//
//    fn fetch(&self, n: isize) -> int { unimplemented!() }
//
//    fn get_tokens(&self, start: isize, stop: isize, types: &IntervalSet) -> Vec<Token> { unimplemented!() }
//
//    fn lazy_init(&self) { unimplemented!() }
//
//    fn setup(&self) { unimplemented!() }
//
//    fn get_token_source(&self) -> TokenSource { unimplemented!() }
//
//    fn set_token_source(&self, tokenSource: TokenSource) { unimplemented!() }

    //todo make this const generic over direction
    fn next_token_on_channel(&mut self, mut i: isize, channel: isize, direction: isize) -> isize {
        self.sync(i);
        if i >= self.size() {
            return self.size() - 1;
        }

        let mut token = self.base.tokens[i as usize].as_ref();
        while token.get_channel() != channel {
            if token.get_token_type() == EOF || i < 0 {
                return i;
            }

            i += direction;
            self.sync(i);
            token = self.base.tokens[i as usize].as_ref();
        }

        return i;
    }
//
//    fn previous_token_on_channel(&self, i: isize, channel: isize) -> int { unimplemented!() }
//
//    fn get_hidden_tokens_to_right(&self, tokenIndex: isize, channel: isize) -> Vec<Token> { unimplemented!() }
//
//    fn get_hidden_tokens_to_left(&self, tokenIndex: isize, channel: isize) -> Vec<Token> { unimplemented!() }
//
//    fn filter_for_channel(&self, left: isize, right: isize, channel: isize) -> Vec<Token> { unimplemented!() }
//
//    fn get_source_name(&self) -> String { unimplemented!() }
//
//    fn get_all_text(&self) -> String { unimplemented!() }
//
//    fn get_text_from_tokens(&self, start: Token, end: Token) -> String { unimplemented!() }
//
//    fn get_text_from_rule_context(&self, interval: RuleContext) -> String { unimplemented!() }
//
//    fn get_text_from_interval(&self, interval: &Interval) -> String { unimplemented!() }
//
//    fn fill(&self) { unimplemented!() }
//
//    fn adjust_seek_index(&self, i: isize) -> int { unimplemented!() }

    fn lb(&mut self, k: isize) -> Option<&dyn Token> {
        if k == 0 || (self.base.p - k) < 0 { return None }

        let mut i = self.base.p;
        let mut n = 1;
        // find k good tokens looking backwards
        while n <= k && i > 0 {
            // skip off-channel tokens
            i = self.next_token_on_channel(i - 1, self.channel, -1);
            n += 1;
        }
        if i < 0 { return None }

        return Some(self.get(i));
    }

//    fn get_number_of_on_channel_tokens(&self) -> int { unimplemented!() }
}
