#![allow(nonstandard_style)]
// Generated from CSV.g4 by ANTLR 4.8
use super::csvparser::*;
use antlr_rust::tree::ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link CSVParser}.
 */
pub trait CSVVisitor<'input>: ParseTreeVisitor<'input, CSVParserContextType> {
    /**
     * Visit a parse tree produced by {@link CSVParser#csvFile}.
     * @param ctx the parse tree
     */
    fn visit_csvFile(&mut self, ctx: &CsvFileContext<'input>) { self.visit_children(ctx) }

    /**
     * Visit a parse tree produced by {@link CSVParser#hdr}.
     * @param ctx the parse tree
     */
    fn visit_hdr(&mut self, ctx: &HdrContext<'input>) { self.visit_children(ctx) }

    /**
     * Visit a parse tree produced by {@link CSVParser#row}.
     * @param ctx the parse tree
     */
    fn visit_row(&mut self, ctx: &RowContext<'input>) { self.visit_children(ctx) }

    /**
     * Visit a parse tree produced by {@link CSVParser#field}.
     * @param ctx the parse tree
     */
    fn visit_field(&mut self, ctx: &FieldContext<'input>) { self.visit_children(ctx) }
}
