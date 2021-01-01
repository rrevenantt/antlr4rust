#[derive(Debug)]
pub struct ATNDeserializationOptions {
    read_only: bool,
    verify_atn: bool,
    generate_rule_bypass_transitions: bool,
}

impl ATNDeserializationOptions {
    pub fn is_verify(&self) -> bool { self.verify_atn }
}

impl Default for ATNDeserializationOptions {
    fn default() -> Self {
        ATNDeserializationOptions {
            read_only: true,
            verify_atn: true,
            generate_rule_bypass_transitions: false,
        }
    }
}
