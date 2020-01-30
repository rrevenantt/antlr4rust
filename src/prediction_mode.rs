use std::collections::HashMap;

use bit_set::BitSet;

use crate::atn::INVALID_ALT;
use crate::atn_config::ATNConfig;
use crate::atn_config_set::ATNConfigSet;
use crate::atn_state::ATNStateRef;
use crate::prediction_context::PredictionContext;
use crate::semantic_context::SemanticContext;

const PREDICTION_MODE_SLL: isize = 0;
const PREDICTION_MODE_LL: isize = 1;
const PREDICTION_MODE_LLEXACT_AMBIG_DETECTION: isize = 2;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PredictionMode {
    SLL = 0,
    LL,
    LL_EXACT_AMBIG_DETECTION,
}

impl PredictionMode {
    //todo move everything here
}

//
//
pub fn has_sll_conflict_terminating_prediction(mode: PredictionMode, configs: &ATNConfigSet) -> bool {
//    if all_configs_in_rule_stop_states(configs) {
//        return true          checked outside
//    }
    let mut dup = ATNConfigSet::new_base_atnconfig_set(true);
    let mut configs = &*configs;
    if mode == PredictionMode::SLL {
        if configs.has_semantic_context() {
            configs.get_items().for_each(|it| {
                let c = ATNConfig::new_with_semantic(
                    it.get_state(),
                    it.get_alt(),
                    it.get_context().cloned(),
                    Some(Box::new(SemanticContext::NONE)),
                );
                dup.add(Box::new(c));
            });
            configs = &dup;
        }
    }

    let altsets = get_conflicting_alt_subsets(&configs);
    let heuristic = has_conflicting_alt_set(&altsets) && !has_state_associated_with_one_alt(&configs);
    return heuristic;
}

//fn all_configs_in_rule_stop_states(configs: &ATNConfigSet) -> bool {
//    for co
//}

pub fn resolves_to_just_one_viable_alt(altsets: &Vec<BitSet>) -> isize {
    get_single_viable_alt(altsets)
}

pub fn all_subsets_conflict(altsets: &Vec<BitSet>) -> bool { !has_non_conflicting_alt_set(altsets) }

pub fn all_subsets_equal(altsets: &Vec<BitSet>) -> bool {
    let mut iter = altsets.iter();
    let first = iter.next();
    iter.all(|it| it == first.unwrap())
}

fn has_non_conflicting_alt_set(altsets: &Vec<BitSet>) -> bool {
    altsets.iter().any(|it| it.len() == 1)
}

fn has_conflicting_alt_set(altsets: &Vec<BitSet>) -> bool {
    for alts in altsets {
        if alts.len() > 1 {
            return true;
        }
    }
    false
}


//fn get_unique_alt(altsets: &Vec<BitSet>) -> int { unimplemented!() }
//
pub fn get_alts(altsets: &Vec<BitSet>) -> BitSet {
    altsets.iter()
        .fold(BitSet::new(), |mut acc, it| {
            acc.extend(it);
            acc
        })
}

//
pub(crate) fn get_conflicting_alt_subsets(configs: &ATNConfigSet) -> Vec<BitSet> {
    let mut configs_to_alts: HashMap<(ATNStateRef, &PredictionContext), BitSet> = HashMap::new();
    for c in configs.get_items() {
        let alts = configs_to_alts.entry((c.get_state(), c.get_context().unwrap()))
            .or_default();

        alts.insert(c.get_alt() as usize);
    }
    configs_to_alts.drain().map(|(_, x)| x).collect()
}

fn get_state_to_alt_map(configs: &ATNConfigSet) -> HashMap<ATNStateRef, BitSet> {
    let mut m = HashMap::new();
    for c in configs.get_items() {
        let alts = m.entry(c.get_state())
            .or_insert(BitSet::new());
        alts.insert(c.get_alt() as usize);
    }
    m
}

fn has_state_associated_with_one_alt(configs: &ATNConfigSet) -> bool {
    let x = get_state_to_alt_map(configs);
    for alts in x.values() {
        if alts.len() == 1 {
            return true
        }
    }
    false
}

pub fn get_single_viable_alt(altsets: &Vec<BitSet>) -> isize {
    let mut viable_alts = BitSet::new();
    let mut min_alt = INVALID_ALT as usize;
    for alt in altsets {
        min_alt = alt.iter().next().unwrap();
        viable_alts.insert(min_alt);
        if viable_alts.len() > 1 {
            return INVALID_ALT
        }
    }
    min_alt as isize
}
