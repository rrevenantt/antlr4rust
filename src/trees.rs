/** A set of utility routines useful for all kinds of ANTLR trees. */

fn trees_String_tree(tree: Tree, ruleNames: Vec<String>, recog: Recognizer) -> String { unimplemented!() }

fn trees_get_node_text(t: Tree, ruleNames: Vec<String>, recog: Parser) -> String { unimplemented!() }

fn trees_get_children(t: Tree) -> Vec<Tree> { unimplemented!() }

fn treesget_ancestors(t: Tree) -> Vec<Tree> { unimplemented!() }

fn trees_find_all_token_nodes(t: ParseTree, ttype: isize) -> Vec<ParseTree> { unimplemented!() }

fn treesfind_all_rule_nodes(t: ParseTree, ruleIndex: isize) -> Vec<ParseTree> { unimplemented!() }

fn treesfind_all_nodes(t: ParseTree, index: isize, findTokens: bool) -> Vec<ParseTree> { unimplemented!() }

fn trees_find_all_nodes(t: ParseTree, index: isize, findTokens: bool, nodes: * Vec<ParseTree>) { unimplemented!() }

fn trees_descendants(t: ParseTree) -> Vec<ParseTree> { unimplemented!() }
