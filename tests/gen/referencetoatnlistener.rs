#![allow(non_snake_case)]

use std::any::Any;

use antlr_rust::parser_rule_context::{cast, ParserRuleContext};
// Generated from ReferenceToATN.g4 by ANTLR 4.7.2
use antlr_rust::tree::ParseTreeListener;

use super::referencetoatnparser::*;

pub trait ReferenceToATNListener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link ReferenceToATNParser#a}.
     * @param ctx the parse tree
     */
    fn enter_a(&mut self, ctx: &AContext) {}
    /**
     * Exit a parse tree produced by {@link ReferenceToATNParser#a}.
     * @param ctx the parse tree
     */
    fn exit_a(&mut self, ctx: &AContext) {}
}
