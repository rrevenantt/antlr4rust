#![allow(nonstandard_style)]
// Generated from Perf.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use antlr_rust::token_factory::CommonTokenFactory;
use super::perfparser::*;

use std::any::Any;

pub trait PerfListener<'input> : ParseTreeListener<'input,PerfParserContextType>{

/**
 * Enter a parse tree produced by {@link PerfParser#stat}.
 * @param ctx the parse tree
 */
fn enter_stat(&mut self, _ctx: &StatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PerfParser#stat}.
 * @param ctx the parse tree
 */
fn exit_stat(&mut self, _ctx: &StatContext<'input>) { }

/**
 * Enter a parse tree produced by {@link PerfParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PerfParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }

}
