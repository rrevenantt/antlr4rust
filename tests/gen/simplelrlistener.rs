#![allow(non_snake_case)]

use std::any::Any;

// Generated from SimpleLR.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;

use super::simplelrparser::*;

pub trait SimpleLRListener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link SimpleLRParser#s}.
     * @param ctx the parse tree
     */
    fn enter_s(&mut self, _ctx: &SContext) {}
    /**
     * Exit a parse tree produced by {@link SimpleLRParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext) {}

    /**
     * Enter a parse tree produced by {@link SimpleLRParser#a}.
     * @param ctx the parse tree
     */
    fn enter_a(&mut self, _ctx: &AContext) {}
    /**
     * Exit a parse tree produced by {@link SimpleLRParser#a}.
     * @param ctx the parse tree
     */
    fn exit_a(&mut self, _ctx: &AContext) {}
}
