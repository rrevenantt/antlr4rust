use crate::recognizer::Recognizer;
use crate::rule_context::RuleContext;

pub trait SemanticContext {
    fn evaluate(&self, parser: &Recognizer, outerContext: &RuleContext) -> bool;
    fn eval_precedence(
        &self,
        parser: &Recognizer,
        outerContext: &RuleContext,
    ) -> Box<SemanticContext>;
}

fn and_context(_a: &SemanticContext, _b: &SemanticContext) -> Box<SemanticContext> {
    unimplemented!()
}

fn or_context(_a: &SemanticContext, _b: &SemanticContext) -> Box<SemanticContext> {
    unimplemented!()
}

pub struct Predicate {
    rule_index: isize,
    pred_index: isize,
    is_ctx_dependent: bool,
}

impl Predicate {
    fn new_predicate(_ruleIndex: isize, _predIndex: isize, _isCtxDependent: bool) -> Predicate {
        unimplemented!()
    }
}

impl SemanticContext for Predicate {
    fn eval_precedence(
        &self,
        _parser: &Recognizer,
        _outerContext: &RuleContext,
    ) -> Box<SemanticContext> {
        unimplemented!()
    }

    fn evaluate(&self, _parser: &Recognizer, _outerContext: &RuleContext) -> bool {
        unimplemented!()
    }
}

pub struct PrecedencePredicate {
    base: Predicate,
    precedence: isize,
}

impl PrecedencePredicate {
    fn new_precedence_predicate(_precedence: isize) -> PrecedencePredicate {
        unimplemented!()
    }

    fn compare_to(&self, _other: &PrecedencePredicate) -> isize {
        unimplemented!()
    }
    //    fn filter_precedence_predicates(set Set) -> &Vec<PrecedencePredicate> { unimplemented!() }
}

impl SemanticContext for PrecedencePredicate {
    fn evaluate(&self, parser: &Recognizer, outerContext: &RuleContext) -> bool {
        self.base.evaluate(parser, outerContext)
    }

    fn eval_precedence(
        &self,
        parser: &Recognizer,
        outerContext: &RuleContext,
    ) -> Box<SemanticContext> {
        self.base.eval_precedence(parser, outerContext)
    }
}
