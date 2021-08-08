#![allow(nonstandard_style)]
// Generated from VisitorBasic.g4 by ANTLR 4.8
use super::visitorbasicparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait VisitorBasicListener<'input>:
    ParseTreeListener<'input, VisitorBasicParserContextType>
{
    /**
     * Enter a parse tree produced by {@link VisitorBasicParser#s}.
     * @param ctx the parse tree
     */
    fn enter_s(&mut self, _ctx: &SContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link VisitorBasicParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : VisitorBasicListener<'input> }
