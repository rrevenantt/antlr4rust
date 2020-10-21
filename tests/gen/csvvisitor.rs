#![allow(nonstandard_style)]
// Generated from CSV.g4 by ANTLR 4.8
use super::csvparser::*;
use antlr_rust::parser_rule_context::RuleContextExt;
use antlr_rust::tree::{ParseTreeVisitor, VisitChildren};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link CSVParser}.
 *
 * @param <T> The return type of the visit operation. Use {@link Void} for
 * operations with no return type.
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

    //	/// By default recursively visits all childrens of the node.
    //	/// Implement it if you want different default visiting logic.
    //	fn visit_children(&mut self, node: &(dyn CSVParserContext<'input> + 'input)) {}
}

//impl<'a,'input,T> CSVVisitor<'input> for T where T: CSVVisitor<'input> + 'a{
//	//	default fn visit_csvFile(&mut self, ctx: &CsvFileContext<'input>){
//		self.visit_children(ctx)
//	}
//	default fn visit_hdr(&mut self, ctx: &HdrContext<'input>){
//		self.visit_children(ctx)
//	}
//	default fn visit_row(&mut self, ctx: &RowContext<'input>){
//		self.visit_children(ctx)
//	}
//	default fn visit_field(&mut self, ctx: &FieldContext<'input>){
//		self.visit_children(ctx)
//	}
//
//	default fn visit_children(&mut self, node: &(dyn CSVParserContext<'input> + 'input)){
//		node.accept_children(self as &mut (dyn CSVVisitor<'input> + 'a))
//	}
//}
