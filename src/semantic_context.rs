use crate::recognizer::Recognizer;
use crate::rule_context::RuleContext;

//pub trait SemanticContext:Sync + Send {
////    fn evaluate(&self, parser: &Recognizer, outerContext: &RuleContext) -> bool;
////    fn eval_precedence(&self, parser: &Recognizer, outerContext: &RuleContext, ) -> Box<dyn SemanticContext>;
//}

fn empty() -> SemanticContext {
    SemanticContext::Predicate {
        rule_index: -1,
        pred_index: -1,
        is_ctx_dependent: false,
    }
}

#[derive(Clone)]
pub enum SemanticContext {
    Predicate {
        rule_index: isize,
        pred_index: isize,
        is_ctx_dependent: bool,
    },
    Precedence(isize),
    AND(Vec<SemanticContext>),
    OR(Vec<SemanticContext>),
}

impl SemanticContext {
    fn evaluate(&self, parser: &Recognizer, outerContext: &RuleContext) -> bool {
        unimplemented!()
    }
    fn eval_precedence(&self, parser: &Recognizer, outerContext: &RuleContext) -> SemanticContext {
        unimplemented!()
    }
}


//fn and_context(_a: &SemanticContext, _b: &SemanticContext) -> Box<SemanticContext> {
//    unimplemented!()
//}
//
//fn or_context(_a: &SemanticContext, _b: &SemanticContext) -> Box<SemanticContext> {
//    unimplemented!()
//}
