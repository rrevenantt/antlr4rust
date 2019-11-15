use crate::atn_config_set::ATNConfigSet;
use bit_set::BitSet;
use std::collections::HashMap;
use crate::atn_state::ATNStateRef;
use crate::prediction_context::PredictionContext;
use crate::atn::INVALID_ALT;

const PREDICTION_MODE_SLL: isize = 0;
const PREDICTION_MODE_LL: isize = 1;
const PREDICTION_MODE_LLEXACT_AMBIG_DETECTION: isize = 2;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PredictionMode {
    SLL = 0,
    LL,
    LL_EXACT_AMBIG_DETECTION,
}

//
//
//fn has_sllconflict_terminating_prediction(mode: isize, configs: ATNConfigSet) -> bool { unimplemented!() }

//fn all_configs_in_rule_stop_states(configs: ATNConfigSet) -> bool { unimplemented!() }

pub fn resolves_to_just_one_viable_alt(altsets: &Vec<BitSet>) -> isize {
    get_single_viable_alt(altsets)
}

//fn all_subsets_conflict(altsets: &Vec<BitSet>) -> bool { unimplemented!() }
//
//fn has_non_conflicting_alt_set(altsets: &Vec<BitSet>) -> bool { unimplemented!() }
//
//fn has_conflicting_alt_set(altsets: &Vec<BitSet>) -> bool { unimplemented!() }
//
//fn all_subsets_equal(altsets: &Vec<BitSet>) -> bool { unimplemented!() }
//
//fn get_unique_alt(altsets: &Vec<BitSet>) -> int { unimplemented!() }
//
//fn _get_alts(altsets: &Vec<BitSet>) -> * BitSet { unimplemented!() }
//
pub(crate) fn get_conflicting_alt_subsets(configs: &ATNConfigSet) -> Vec<BitSet> {
    let mut configs_to_alts: HashMap<(ATNStateRef, &PredictionContext), BitSet> = HashMap::new();
    for c in configs.get_items() {
        let alts = configs_to_alts.entry((c.get_state(), c.get_context().unwrap()))
            .or_insert(BitSet::new());

        alts.insert(c.get_alt() as usize);
    }
    configs_to_alts.drain().map(|(_, x)| x).collect()
}
//
//fn _get_state_to_alt_map(configs: ATNConfigSet) -> * AltDict { unimplemented!() }
//
//fn has_state_associated_with_one_alt(configs: ATNConfigSet) -> bool { unimplemented!() }

fn get_single_viable_alt(altsets: &Vec<BitSet>) -> isize {
    let viable_alt = altsets.first().unwrap().iter().next().unwrap();
    for alt in altsets {
        let min_alt = alt.iter().next().unwrap();
        if min_alt < viable_alt {
            return INVALID_ALT
        }
    }
    viable_alt as isize
}
