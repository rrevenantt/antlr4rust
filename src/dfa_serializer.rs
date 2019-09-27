use crate::dfa::DFA;
use std::fmt::{Display, Formatter};
use crate::dfa_state::DFAState;

pub struct DFASerializer<'a, 'b> {
    dfa: &'a DFA,
    get_edge_label: &'b dyn Fn(usize) -> String,
}

impl Display for DFASerializer<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dfa = self.dfa.states.read().unwrap();
        for source in dfa.iter() {
            for (i, edge) in source.edges.iter().copied().enumerate() {
                if edge != 0 && edge != usize::max_value() {
                    let target = &dfa[edge];
                    f.write_fmt(format_args!("{}-{}->{}\n",
                                             self.get_state_string(source),
                                             (self.get_edge_label)(i),
                                             self.get_state_string(target)
                    ));
                }
            }
        }
        Ok(())
    }
}

impl DFASerializer<'_, '_> {
    pub fn new<'a, 'b>(dfa: &'a DFA, get_edge_label: &'b dyn Fn(usize) -> String) -> DFASerializer<'a, 'b> {
        DFASerializer {
            dfa,
            get_edge_label,
        }
    }

    fn get_state_string(&self, state: &DFAState) -> String {
        let mut base_str = format!("{}s{}{}",
                                   if state.is_accept_state { ":" } else { "" },
                                   state.state_number - 1,
                                   if state.requires_full_context { "^" } else { "" },
        );
        if state.is_accept_state {
            base_str = if (false /*TODO predicates*/) { unimplemented!() } else { format!("{}=>{}", base_str, state.prediction) };
        }
        base_str
    }
}
