#![allow(non_snake_case)]
// Generated from CSV.g4 by ANTLR 4.7.1
use antlr_rust::tree::{ParseTreeListener, ListenerDispatch};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::parser::ListenerCaller;
use super::csvparser::*;

pub trait CSVListener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link CSVParser#csvFile}.
     * @param ctx the parse tree
     */
    fn enter_csvFile(&mut self, ctx: &CsvFileContext) {}
    /**
     * Exit a parse tree produced by {@link CSVParser#csvFile}.
     * @param ctx the parse tree
     */
    fn exit_csvFile(&mut self, ctx: &CsvFileContext) {}

    /**
     * Enter a parse tree produced by {@link CSVParser#hdr}.
     * @param ctx the parse tree
     */
    fn enter_hdr(&mut self, ctx: &HdrContext) {}
    /**
     * Exit a parse tree produced by {@link CSVParser#hdr}.
     * @param ctx the parse tree
     */
    fn exit_hdr(&mut self, ctx: &HdrContext) {}

    /**
     * Enter a parse tree produced by {@link CSVParser#row}.
     * @param ctx the parse tree
     */
    fn enter_row(&mut self, ctx: &RowContext) {}
    /**
     * Exit a parse tree produced by {@link CSVParser#row}.
     * @param ctx the parse tree
     */
    fn exit_row(&mut self, ctx: &RowContext) {}

    /**
     * Enter a parse tree produced by {@link CSVParser#field}.
     * @param ctx the parse tree
     */
    fn enter_field(&mut self, ctx: &FieldContext) {}
    /**
     * Exit a parse tree produced by {@link CSVParser#field}.
     * @param ctx the parse tree
     */
    fn exit_field(&mut self, ctx: &FieldContext) {}
}

pub struct CSVListenerCaller;

impl ListenerCaller<dyn CSVListener> for CSVListenerCaller {
    fn enter_rule(ctx: &dyn ParserRuleContext, listener: &mut dyn CSVListener) {
        listener.enter_every_rule(ctx);
        match ctx.get_rule_index() {
            RULE_csvFile => listener.enter_csvFile(unsafe { &*(ctx as *const dyn ParserRuleContext as *const CsvFileContext) }),

            RULE_hdr => listener.enter_hdr(unsafe { &*(ctx as *const dyn ParserRuleContext as *const HdrContext) }),

            RULE_row => listener.enter_row(unsafe { &*(ctx as *const dyn ParserRuleContext as *const RowContext) }),

            RULE_field => listener.enter_field(unsafe { &*(ctx as *const dyn ParserRuleContext as *const FieldContext) }),

            _ => panic!("invalid rule")
        }
    }

    fn exit_rule(ctx: &dyn ParserRuleContext, listener: &mut dyn CSVListener) {
        listener.exit_every_rule(ctx);
        match ctx.get_rule_index() {
            RULE_csvFile => listener.exit_csvFile(unsafe { &*(ctx as *const dyn ParserRuleContext as *const CsvFileContext) }),

            RULE_hdr => listener.exit_hdr(unsafe { &*(ctx as *const dyn ParserRuleContext as *const HdrContext) }),

            RULE_row => listener.exit_row(unsafe { &*(ctx as *const dyn ParserRuleContext as *const RowContext) }),

            RULE_field => listener.exit_field(unsafe { &*(ctx as *const dyn ParserRuleContext as *const FieldContext) }),

            _ => panic!("invalid rule")
        }
    }
}
