use std::fmt::Debug;
use std::marker::Unsize;
use std::ops::Deref;

use crate::char_stream::CharStream;
use crate::int_stream::{IntStream, EOF};
use crate::token::{Token, TOKEN_DEFAULT_CHANNEL};
use crate::token_factory::{TokenAware, TokenFactory};

/// Provides tokens for parser via `TokenStream`
pub trait TokenSource<'input>: TokenAware<'input> {
    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok;
    /**
     * Get the line number for the current position in the input stream. The
     * first line in the input is line 1.
     *
     * @return The line number for the current position in the input stream, or
     * 0 if the current token source does not track line numbers.
     */
    fn get_line(&self) -> isize;
    /**
     * Get the index into the current line for the current position in the input
     * stream. The first character on a line has position 0.
     *
     * @return The line number for the current position in the input stream, or
     * -1 if the current token source does not track character positions.
     */
    fn get_char_position_in_line(&self) -> isize;
    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream>;
    fn get_source_name(&self) -> String;
    //    fn set_token_factory<'c: 'b>(&mut self, f: &'c TokenFactory);
    /// Gets the {@link TokenFactory} this token source is currently using for
    /// creating {@link Token} objects from the input.
    ///
    /// Required by `Parser` for creating missing tokens.
    ///
    /// @return The {@link TokenFactory} currently used by this token source.
    fn get_token_factory(&self) -> &'input Self::TF;
}

impl<'input, T> TokenAware<'input> for &mut T
where
    T: TokenSource<'input>,
{
    type TF = T::TF;
}

// allows user to call parser with &mut reference to Lexer
impl<'input, T> TokenSource<'input> for &mut T
where
    T: TokenSource<'input>,
{
    #[inline(always)]
    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok { (**self).next_token() }

    #[inline(always)]
    fn get_line(&self) -> isize { (**self).get_line() }

    #[inline(always)]
    fn get_char_position_in_line(&self) -> isize { (**self).get_char_position_in_line() }

    #[inline(always)]
    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> { (**self).get_input_stream() }

    #[inline(always)]
    fn get_source_name(&self) -> String { (**self).get_source_name() }

    #[inline(always)]
    fn get_token_factory(&self) -> &'input Self::TF { (**self).get_token_factory() }
}

// / adaptor to feed parser with existing tokens
// pub struct IterTokenSource<S, F> where S: Iterator, S::Item: Token, F: TokenFactory<Tok=S::Item> {
//     iter: S,
//     fact: F,
// }
//
// impl<S, F> TokenSource for IterTokenSource<S, F> where S: Iterator, S::Item: Token, F: TokenFactory<Tok=S::Item> {
//     type Tok = S::Item;
//
//     fn next_token(&mut self) -> Box<Self::Tok> {
//         self.iter.next().map(Box::new).unwrap_or_else(
//             || self.get_token_factory().create(
//                 None,
//                 EOF,
//                 TOKEN_DEFAULT_CHANNEL,
//                 -1,
//                 -1,
//                 self.get_line(),
//                 self.get_char_position_in_line(),
//             )
//         )
//     }
//
//     fn get_line(&self) -> isize {
//         0
//     }
//
//     fn get_char_position_in_line(&self) -> isize {
//         -1
//     }
//
//     fn get_input_stream(&mut self) -> Option<&mut dyn CharStream> {
//         None
//     }
//
//     fn get_source_name(&self) -> String {
//         "<iterator>".to_string()
//     }
//
//     fn get_token_factory(&self) -> &dyn TokenFactory<Tok=Self::Tok> {
//         &self.fact
//     }
// }
