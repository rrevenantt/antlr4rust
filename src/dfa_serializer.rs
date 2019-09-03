impl DFASerializer {
    fn get_edge_label(&self, i: isize) -> String { unimplemented!() }

    fn get_state_String(&self, s: * DFAState) -> String { unimplemented!() }
}


pub struct BaseDFASerializer {
    dfa: * DFA,
    literal_names: Vec<String>,
    symbolic_names: Vec<String>,
}

impl BaseDFASerializer {
    fn new(dfa: * DFA, literalNames: Vec<String>, symbolicNames: Vec<String>) -> DFASerializer { unimplemented!() }
}

impl DFASerializer for BaseDFASerializer {
    fn get_edge_label(&self, i: isize) -> String { unimplemented!() }

    fn get_state_String(&self, s: * DFAState) -> String { unimplemented!() }
}
 