/*!
A set of utility routines useful for all kinds of ANTLR trees.
*/


use std::ops::Deref;

use crate::tree::Tree;
use crate::utils;

pub fn string_tree(tree: &(impl Tree + ?Sized), rule_names: &[&str]) -> String {
    let s = utils::escape_whitespaces(get_node_text(tree, rule_names), false);
    if tree.get_child_count() == 0 { return s }
    let mut result = String::new();
    result.push('(');
    result.extend(s.chars());
    result = tree.get_children()
        .iter()
        .map(|child| string_tree(child.deref(), rule_names))
        .fold(result, |mut acc, text| {
            acc.push(' ');
            acc.extend(text.chars());
            acc
        });
    result.push(')');
    result
}


pub fn get_node_text(t: &(impl Tree + ?Sized), rule_names: &[&str]) -> String {
    t.get_node_text(rule_names)
}

//pub fn get_children(t: impl Tree) -> Vec<Rc<dyn Tree>> { unimplemented!() }
//
//pub fn get_ancestors(t: impl Tree) -> Vec<Rc<dyn Tree>> { unimplemented!() }
//
//pub fn find_all_token_nodes(t: impl ParseTree, ttype: isize) -> Vec<Rc<dyn ParseTree>> { unimplemented!() }
//
//pub fn find_all_rule_nodes(t: impl ParseTree, rule_index: isize) -> Vec<Rc<dyn ParseTree>> { unimplemented!() }
//
//pub fn find_all_nodes(t: impl ParseTree, index: isize, find_tokens: bool) -> Vec<Rc<dyn ParseTree>> { unimplemented!() }
//
////fn trees_find_all_nodes(t: ParseTree, index: isize, findTokens: bool, nodes: * Vec<ParseTree>) { unimplemented!() }
//
//pub fn descendants(t: impl ParseTree) -> Vec<dyn ParseTree> { unimplemented!() }
