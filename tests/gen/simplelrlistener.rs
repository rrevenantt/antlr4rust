#![allow(non_snake_case)]

use std::any::Any;

use antlr_rust::parser::ListenerCaller;
use antlr_rust::parser_rule_context::{cast, ParserRuleContext};
// Generated from SimpleLR.g4 by ANTLR 4.7.2
use antlr_rust::tree::ParseTreeListener;

use super::simplelrparser::*;

pub trait SimpleLRListener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link SimpleLRParser#s}.
     * @param ctx the parse tree
     */
    fn enter_s(&mut self, ctx: &SContext) {}
    /**
     * Exit a parse tree produced by {@link SimpleLRParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, ctx: &SContext) {}

    /**
     * Enter a parse tree produced by {@link SimpleLRParser#a}.
     * @param ctx the parse tree
     */
    fn enter_a(&mut self, ctx: &AContext) {}
    /**
     * Exit a parse tree produced by {@link SimpleLRParser#a}.
     * @param ctx the parse tree
     */
    fn exit_a(&mut self, ctx: &AContext) {}
}

pub struct SimpleLRListenerCaller;

impl ListenerCaller<dyn SimpleLRListener> for SimpleLRListenerCaller {
    fn enter_rule(ctx: &dyn ParserRuleContext, listener: &mut Box<dyn SimpleLRListener>) {
        listener.enter_every_rule(ctx);
        ctx.enter_rule(listener as &mut dyn Any);
    }

    fn exit_rule(ctx: &dyn ParserRuleContext, listener: &mut Box<dyn SimpleLRListener>) {
        listener.exit_every_rule(ctx);
        ctx.exit_rule(listener as &mut dyn Any);
    }
}
