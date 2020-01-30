use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::hash::{Hash, Hasher};

use bit_set::BitSet;
use murmur3::murmur3_32::MurmurHasher;

use crate::atn_config::ATNConfig;
use crate::atn_simulator::IATNSimulator;
use crate::parser_atn_simulator::MergeCache;
use crate::prediction_context::PredictionContext;
use crate::semantic_context::SemanticContext;

//pub trait ATNConfigSet:Sync+Send{
//    fn hash(&self) ->isize;
//    fn add_cached(&mut self, config: Box<dyn ATNConfig>, merge_cache: Option<&HashMap<(PredictionContext,PredictionContext),PredictionContext>>) -> bool;
//    fn add(&mut self, config: Box<dyn ATNConfig>) -> &dyn ATNConfig;
//    fn add_all(&mut self, coll: Vec<&dyn ATNConfig>) -> bool;
//
////    fn get_states(&self) -> * Set;
//    fn get_predicates(&self) -> Vec<&dyn SemanticContext>;
//    fn get_items<T:Iterator<Item=&dyn ATNConfig>>(&self) -> T;
//
////    fn optimize_configs(&self, interpreter: &BaseATNSimulator);
//
//    fn equals(&self, other: &dyn ATNConfigSet) ->bool;
//
//    fn length(&self) -> isize;
//    fn is_empty(&self) -> bool;
//    fn contains(&self, item: &dyn ATNConfig) -> bool;
//    fn contains_fast(&self, item: &dyn ATNConfig) -> bool;
//    fn clear(&self);
//    fn String(&self) -> String;
//
//    fn has_semantic_context(&self) -> bool;
//    fn set_has_semantic_context(&mut self, v: bool);
//
//    fn read_only(&self) -> bool;
//    fn set_read_only(&self, read_only: bool);
//
////    fn get_conflicting_alts(&self) -> * BitSet;
////    fn set_conflicting_alts(&self, v: * BitSet);
//
//    fn full_context(&self) -> bool;
//
//    fn get_unique_alt(&self) -> isize;
//    fn set_unique_alt(&self, v: isize);
//
//    fn get_dips_into_outer_context(&self) -> bool;
//    fn set_dips_into_outer_context(&self, v: bool);
//}

//#[derive(Debug)]
pub struct ATNConfigSet {
    cached_hash: u64,

    config_lookup: HashMap<u64, usize>,

    //todo remove box?
    pub(crate) configs: Vec<Box<ATNConfig>>,

    pub(crate) conflicting_alts: BitSet,

    dips_into_outer_context: bool,

    full_ctx: bool,

    has_semantic_context: bool,

    read_only: bool,

    unique_alt: isize,

    hasher: fn(&ATNConfig) -> u64,
}

impl Debug for ATNConfigSet {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}

impl PartialEq for ATNConfigSet {
    fn eq(&self, other: &Self) -> bool {
        self.configs == other.configs &&
            self.full_ctx == other.full_ctx &&
            self.unique_alt == other.unique_alt &&
            self.conflicting_alts == other.conflicting_alts &&
            self.has_semantic_context == other.has_semantic_context &&
            self.dips_into_outer_context == other.dips_into_outer_context
    }
}

impl Eq for ATNConfigSet {}

impl Hash for ATNConfigSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
//        if self.cached_hash.get() == 0  {
        self.configs.hash(state)
//        }

//        state.write_u64(self.cached_hash.get())
    }
}

impl ATNConfigSet {
    pub fn new_base_atnconfig_set(full_ctx: bool) -> ATNConfigSet {
        ATNConfigSet {
            cached_hash: 0,
            config_lookup: HashMap::new(),
            configs: vec![],
            conflicting_alts: Default::default(),
            dips_into_outer_context: false,
            full_ctx,
            has_semantic_context: false,
            read_only: false,
            unique_alt: 0,
            hasher: Self::atn_config_local_hash
        }
    }

    fn hash_code_configs(&self) -> isize {
        unimplemented!()
    }

    pub fn new_ordered_atnconfig_set() -> ATNConfigSet {
        let mut a = ATNConfigSet::new_base_atnconfig_set(true);

        a.hasher = Self::atn_config_full_hash;
//        unimplemented!();
        a
    }

    fn equal_atnconfigs() -> bool {
        unimplemented!()
    }
    //}
    //
    //impl ATNConfigSet for BaseATNConfigSet {

    //    fn add(&self, config: ATNConfig, mergeCache: * DoubleDict) -> bool { unimplemented!() }
    fn atn_config_full_hash(config: &ATNConfig) -> u64 {
        let mut hashcode = 7u64;
        hashcode = 31 * hashcode + config.get_state() as u64;
        hashcode = 31 * hashcode + config.get_alt() as u64;
        let mut hasher = MurmurHasher::default();
        config.get_context().hash(&mut hasher);
        config.get_semantic_context().hash(&mut hasher);
        hashcode = 31 * hashcode + hasher.finish();

        hashcode
    }


    fn atn_config_local_hash(config: &ATNConfig) -> u64 {
        let mut hashcode = 7u64;
        hashcode = 31 * hashcode + config.get_state() as u64;
        hashcode = 31 * hashcode + config.get_alt() as u64;
        let mut hasher = MurmurHasher::default();
        config.get_semantic_context().hash(&mut hasher);
        hashcode = 31 * hashcode + hasher.finish();

        hashcode
    }

    pub fn add_cached(
        &mut self,
        mut config: Box<ATNConfig>,
        _merge_cache: Option<&mut MergeCache>,
    ) -> bool {
        assert!(!self.read_only);

        if config.get_semantic_context().is_some() && *config.get_semantic_context().unwrap() != SemanticContext::NONE {
            self.has_semantic_context = true
        }

//        assert!(config.get_context().unwrap().is_consistent());

        if config.get_reaches_into_outer_context() > 0 {
            self.dips_into_outer_context = true
        }
        let hasher = self.hasher;
        let hash = hasher(config.as_ref());

        if let Some(existing) = self.config_lookup.get(&hash) {
            let existing = self.configs.get_mut(*existing).unwrap().as_mut();
            let root_is_wildcard = !self.full_ctx;
            let mut merged = PredictionContext::merge(
                existing.take_context(),
                config.take_context(),
                root_is_wildcard,
            );

            merged.calc_hash();

            existing.set_reaches_into_outer_context(
                max(existing.get_reaches_into_outer_context(), config.get_reaches_into_outer_context())
            );

            if config.is_precedence_filter_suppressed() {
                existing.set_precedence_filter_suppressed(true)
            }

            existing.set_context(merged);
        } else {
            self.config_lookup.insert(hash, self.configs.len());
            self.cached_hash = 0;
            self.configs.push(config);
        }
        true
    }

    //    pub fn get_states(&self) -> * Set { unimplemented!() }

    pub fn add(&mut self, config: Box<ATNConfig>) -> bool {
        self.add_cached(config, None)
    }

    pub fn add_all(&mut self, _coll: Vec<&ATNConfig>) -> bool {
        unimplemented!()
    }

    pub fn get_predicates(&self) -> Vec<&SemanticContext> {
        unimplemented!()
    }

    pub fn get_items(&self) -> impl Iterator<Item=&ATNConfig> {
        self.configs.iter().map(|c| c.as_ref())
    }

    pub fn optimize_configs(&self, _interpreter: &dyn IATNSimulator) {
        //todo
    }

    pub fn equals(&self, _other: &ATNConfigSet) -> bool {
        unimplemented!()
    }

    pub fn length(&self) -> usize {
        self.configs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }

    pub fn contains(&self, _item: &ATNConfig) -> bool {
        unimplemented!()
    }

    pub fn contains_fast(&self, _item: &ATNConfig) -> bool {
        unimplemented!()
    }

    pub fn clear(&self) {
        unimplemented!()
    }

    pub fn string(&self) -> String {
        unimplemented!()
    }

    pub fn has_semantic_context(&self) -> bool {
        self.has_semantic_context
    }

    pub fn set_has_semantic_context(&mut self, _v: bool) {
        self.has_semantic_context = _v;
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn set_read_only(&mut self, _read_only: bool) {
        self.read_only = _read_only;
    }

    pub fn full_context(&self) -> bool {
        self.full_ctx
    }

    //duplicate of the self.conflicting_alts???
    pub fn get_alts(&self) -> BitSet {
        self.configs
            .iter()
            .fold(BitSet::new(), |mut acc, c| {
                acc.insert(c.get_alt() as usize);
                acc
            })
    }

    pub fn get_unique_alt(&self) -> isize {
        self.unique_alt
    }

    pub fn set_unique_alt(&mut self, _v: isize) {
        self.unique_alt = _v
    }

    pub fn get_dips_into_outer_context(&self) -> bool {
        self.dips_into_outer_context
    }

    pub fn set_dips_into_outer_context(&mut self, _v: bool) {
        self.dips_into_outer_context = _v
    }
}
