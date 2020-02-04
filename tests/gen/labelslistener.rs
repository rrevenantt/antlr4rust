#![allow(non_snake_case)]

use antlr_rust::parser::ListenerCaller;
use antlr_rust::parser_rule_context::{cast, ParserRuleContext};
// Generated from Labels.g4 by ANTLR 4.7.2
use antlr_rust::tree::ParseTreeListener;
use std::any::Any;

use super::labelsparser::*;

pub trait LabelsListener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link LabelsParser#z}.
     * @param ctx the parse tree
     */
    fn enter_z(&mut self, ctx: &ZContext) {}
    /**
     * Exit a parse tree produced by {@link LabelsParser#z}.
     * @param ctx the parse tree
     */
    fn exit_z(&mut self, ctx: &ZContext) {}

    /**
     * Enter a parse tree produced by {@link LabelsParser#s}.
     * @param ctx the parse tree
     */
    fn enter_s(&mut self, ctx: &SContext) {}
    /**
     * Exit a parse tree produced by {@link LabelsParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, ctx: &SContext) {}

    /**
     * Enter a parse tree produced by the {@code add}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn enter_add(&mut self, ctx: &AddContext) {}
    /**
     * Exit a parse tree produced by the {@code add}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn exit_add(&mut self, ctx: &AddContext) {}

    /**
     * Enter a parse tree produced by the {@code parens}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn enter_parens(&mut self, ctx: &ParensContext) {}
    /**
     * Exit a parse tree produced by the {@code parens}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn exit_parens(&mut self, ctx: &ParensContext) {}

    /**
     * Enter a parse tree produced by the {@code mult}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn enter_mult(&mut self, ctx: &MultContext) {}
    /**
     * Exit a parse tree produced by the {@code mult}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn exit_mult(&mut self, ctx: &MultContext) {}

    /**
     * Enter a parse tree produced by the {@code dec}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn enter_dec(&mut self, ctx: &DecContext) {}
    /**
     * Exit a parse tree produced by the {@code dec}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn exit_dec(&mut self, ctx: &DecContext) {}

    /**
     * Enter a parse tree produced by the {@code anID}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn enter_anID(&mut self, ctx: &AnIDContext) {}
    /**
     * Exit a parse tree produced by the {@code anID}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn exit_anID(&mut self, ctx: &AnIDContext) {}

    /**
     * Enter a parse tree produced by the {@code anInt}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn enter_anInt(&mut self, ctx: &AnIntContext) {}
    /**
     * Exit a parse tree produced by the {@code anInt}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn exit_anInt(&mut self, ctx: &AnIntContext) {}

    /**
     * Enter a parse tree produced by the {@code inc}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn enter_inc(&mut self, ctx: &IncContext) {}
    /**
     * Exit a parse tree produced by the {@code inc}
     * labeled alternative in {@link LabelsParser#e}.
     * @param ctx the parse tree
     */
    fn exit_inc(&mut self, ctx: &IncContext) {}
}

pub struct LabelsListenerCaller;

impl ListenerCaller<dyn LabelsListener> for LabelsListenerCaller {
    fn enter_rule(ctx: &dyn ParserRuleContext, listener: &mut Box<dyn LabelsListener>) {
        listener.enter_every_rule(ctx);
        ctx.enter_rule(listener as &mut dyn Any);
    }

    fn exit_rule(ctx: &dyn ParserRuleContext, listener: &mut Box<dyn LabelsListener>) {
        listener.exit_every_rule(ctx);
        ctx.exit_rule(listener as &mut dyn Any);
    }
}
