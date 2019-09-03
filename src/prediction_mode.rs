const prediction_mode_sll: isize = 0;
const prediction_mode_ll: isize = 1;
const prediction_mode_llexact_ambig_detection: isize = 2;


fn has_sllconflict_terminating_prediction(mode: isize, configs: ATNConfigSet) -> bool { unimplemented!() }

fn has_config_in_rule_stop_state(configs: ATNConfigSet) -> bool { unimplemented!() }

fn all_configs_in_rule_stop_states(configs: ATNConfigSet) -> bool { unimplemented!() }

fn resolves_to_just_one_viable_alt(altsets: &Vec<BitSet>) -> int { unimplemented!() }

fn all_subsets_conflict(altsets: &Vec<BitSet>) -> bool { unimplemented!() }

fn has_non_conflicting_alt_set(altsets: &Vec<BitSet>) -> bool { unimplemented!() }

fn has_conflicting_alt_set(altsets: &Vec<BitSet>) -> bool { unimplemented!() }

fn all_subsets_equal(altsets: &Vec<BitSet>) -> bool { unimplemented!() }

fn get_unique_alt(altsets: &Vec<BitSet>) -> int { unimplemented!() }

fn _get_alts(altsets: &Vec<BitSet>) -> * BitSet { unimplemented!() }

fn get_conflicting_alt_subsets(configs: ATNConfigSet) -> &Vec<BitSet> { unimplemented!() }

fn _get_state_to_alt_map(configs: ATNConfigSet) -> * AltDict { unimplemented!() }

fn has_state_associated_with_one_alt(configs: ATNConfigSet) -> bool { unimplemented!() }

fn get_single_viable_alt(altsets: &Vec<BitSet>) -> int { unimplemented!() }
