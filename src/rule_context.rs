//use tree::RuleNode;

//pub trait RuleContext:RuleNode {
pub trait RuleContext {
    fn get_invoking_state(&self) -> isize;
    fn set_invoking_state(&self, t: isize);

    fn get_rule_index(&self) -> isize;
    fn is_empty(&self) -> bool;

    fn get_alt_number(&self) -> isize;
    fn set_alt_number(&self, altNumber: isize);
    fn get_parent_ctx(&mut self) -> &mut Option<Box<dyn RuleContext>>;
}

pub trait CustomRuleContext {
    fn get_rule_index(&self) -> usize;
}

pub struct BaseRuleContext {
    parent_ctx: Option<Box<dyn RuleContext>>,
    invoking_state: isize,
    rule_index: isize,
}

impl BaseRuleContext {
    pub(crate) fn new(parent: Option<Box<dyn RuleContext>>, invoking_state: isize) -> Box<dyn RuleContext> {
        Box::new(BaseRuleContext {
            parent_ctx: parent,
            invoking_state,
            rule_index: 0,
        }) as Box<dyn RuleContext>
    }
}

impl RuleContext for BaseRuleContext {
    fn get_invoking_state(&self) -> isize {
        self.invoking_state
    }

    fn set_invoking_state(&self, t: isize) {
        unimplemented!()
    }

    fn get_rule_index(&self) -> isize {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn get_alt_number(&self) -> isize {
        unimplemented!()
    }

    fn set_alt_number(&self, altNumber: isize) {
        unimplemented!()
    }

    fn get_parent_ctx(&mut self) -> &mut Option<Box<RuleContext>> {
        &mut self.parent_ctx
    }
}