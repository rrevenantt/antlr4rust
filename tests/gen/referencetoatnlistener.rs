#![allow(non_snake_case)]

use std::any::Any;

// Generated from ReferenceToATN.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;

use super::referencetoatnparser::*;

pub trait ReferenceToATNListener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link ReferenceToATNParser#a}.
     * @param ctx the parse tree
     */
    fn enter_a(&mut self, _ctx: &AContext) {}
    /**
     * Exit a parse tree produced by {@link ReferenceToATNParser#a}.
     * @param ctx the parse tree
     */
    fn exit_a(&mut self, _ctx: &AContext) {}
}
