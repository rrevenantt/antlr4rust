use rule_context::*;

pub trait ParserRuleContext: RuleContext {
    fn set_exception(&self, e: RecognitionError);

    fn add_token_node(&self, token: Token) -> * TerminalNodeImpl;
    fn add_error_node(&self, badToken: Token) -> * ErrorNodeImpl;

    fn enter_rule(&self, listener: ParseTreeListener);
    fn exit_rule(&self, listener: ParseTreeListener);

    fn set_start(&self, t: Token);
    fn get_start(&self) -> Token;

    fn set_stop(&self, t: Token);
    fn get_stop(&self) -> Token;

    fn add_child(&self, child: RuleContext) -> RuleContext;
    fn remove_last_child(&self);
}

pub struct BaseParserRuleContext {
    base: BaseRuleContext,

    start: Token,
    stop: Token,
    exception: RecognitionError,
    children: Vec<Tree>,
}

impl BaseParserRuleContext {
    fn new_base_parser_rule_context(parent: ParserRuleContext, invokingStateNumber: isize) -> * BaseParserRuleContext { unimplemented!() }

    fn set_exception(&self, e: RecognitionError) { unimplemented!() }

    fn get_children(&self) -> Vec<Tree> { unimplemented!() }

    fn copy_from(&self, ctx: * BaseParserRuleContext) { unimplemented!() }

    fn get_text(&self) -> String { unimplemented!() }

    fn enter_rule(&self, listener: ParseTreeListener) { unimplemented!() }

    fn exit_rule(&self, listener: ParseTreeListener) { unimplemented!() }

    fn add_terminal_node_child(&self, child: TerminalNode) -> TerminalNode { unimplemented!() }

    fn add_child(&self, child: RuleContext) -> RuleContext { unimplemented!() }

    fn remove_last_child(&self) { unimplemented!() }

    fn add_token_node(&self, token: Token) -> * TerminalNodeImpl { unimplemented!() }

    fn add_error_node(&self, badToken: Token) -> * ErrorNodeImpl { unimplemented!() }

    fn get_child(&self, i: isize) -> Tree { unimplemented!() }

    fn get_child_of_type(&self, i: isize, childType: reflect.Type) -> RuleContext { unimplemented!() }

    fn to_String_tree(&self, ruleNames Vec<String>, recog: Recognizer) -> String { unimplemented!() }

    fn get_rule_context(&self) -> RuleContext { unimplemented!() }

    fn accept(&self, visitor: ParseTreeVisitor) -> interface { unimplemented!() } {
    return visitor.VisitChildren(prc)
    }

    fn set_start(&self, t: Token) { unimplemented!() }

    fn get_start(&self) -> Token { unimplemented!() }

    fn set_stop(&self, t: Token) { unimplemented!() }

    fn get_stop(&self) -> Token { unimplemented!() }

    fn get_token(&self, ttype: isize, i: isize) -> TerminalNode { unimplemented!() }

    fn get_tokens(&self, ttype: isize) -> Vec<TerminalNode> { unimplemented!() }

    fn get_payload(&self) -> interface { unimplemented!() } {
    return: prc,
    }

    fn get_child(&self, ctxType: reflect.Type, i: isize) -> RuleContext { unimplemented!() }


    fn get_typed_rule_context(&self, ctxType: reflect.Type, i: isize) -> RuleContext { unimplemented!() }

    fn get_typed_rule_contexts(&self, ctxType: reflect.Type) -> Vec<RuleContext> { unimplemented!() }

    fn get_child_count(&self) -> int { unimplemented!() }

    fn get_source_interval(&self) -> * Interval { unimplemented!() }


    fn String(&self, ruleNames Vec<String>, stop: RuleContext) -> String { unimplemented!() }

    var RuleContextEmpty = NewBaseParserRuleContext(nil, - 1)

    pub trait InterpreterRuleContext {
    parser_rule_context
    }

    pub struct BaseInterpreterRuleContext {
    base: BaseParserRuleContext,
    }

    fn new_base_interpreter_rule_context(parent BaseInterpreterRuleContext, invokingStateNumber: isize, ruleIndex: isize) -> * BaseInterpreterRuleContext { unimplemented!() }
}
 