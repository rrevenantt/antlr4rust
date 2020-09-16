#![allow(nonstandard_style)]

use std::any::Any;

use antlr_rust::token_factory::CommonTokenFactory;
// Generated from ReferenceToATN.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;

use super::referencetoatnparser::*;

pub trait ReferenceToATNListener<'input> : ParseTreeListener<'input,ReferenceToATNParserContextType>{

/**
 * Enter a parse tree produced by {@link ReferenceToATNParser#a}.
 * @param ctx the parse tree
 */
fn enter_a(&mut self, _ctx: &AContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ReferenceToATNParser#a}.
 * @param ctx the parse tree
 */
fn exit_a(&mut self, _ctx: &AContext<'input>) { }

}
