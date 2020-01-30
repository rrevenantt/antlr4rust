#![allow(non_snake_case)]

use std::any::Any;

use antlr_rust::parser_rule_context::{cast, ParserRuleContext};
// Generated from CSV.g4 by ANTLR 4.7.2
use antlr_rust::tree::ParseTreeListener;

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
