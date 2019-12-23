#![allow(non_snake_case)]

use antlr_rust::parser::ListenerCaller;
use antlr_rust::parser_rule_context::ParserRuleContext;
// Generated from ReferenceToATN.g4 by ANTLR 4.7.1
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

pub struct ReferenceToATNListenerCaller;

impl ListenerCaller<dyn ReferenceToATNListener> for ReferenceToATNListenerCaller {
    fn enter_rule(ctx: &dyn ParserRuleContext, listener: &mut dyn ReferenceToATNListener) {
        listener.enter_every_rule(ctx);
        match ctx.get_rule_index() {
            RULE_a => listener.enter_a(unsafe { &*(ctx as *const dyn ParserRuleContext as *const AContext) }),

            _ => panic!("invalid rule")
        }
    }

    fn exit_rule(ctx: &dyn ParserRuleContext, listener: &mut dyn ReferenceToATNListener) {
        listener.exit_every_rule(ctx);
        match ctx.get_rule_index() {
            RULE_a => listener.exit_a(unsafe { &*(ctx as *const dyn ParserRuleContext as *const AContext) }),

            _ => panic!("invalid rule")
        }
    }
}
