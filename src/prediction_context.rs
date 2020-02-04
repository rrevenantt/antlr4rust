use std::collections::{HashMap, LinkedList};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::Deref;
use std::ptr;

use murmur3::murmur3_32::MurmurHasher;

use crate::atn::ATN;
use crate::parser_rule_context::{empty_ctx, ParserRuleContext};
use crate::prediction_context::PredictionContext::{Array, Singleton};
use crate::transition::{RuleTransition, TransitionType};

pub const PREDICTION_CONTEXT_EMPTY_RETURN_STATE: isize = 0x7FFFFFFF;

//pub trait PredictionContext: Sync + Send {
//    fn get_parent(&self, index: isize) -> Option<&BasePredictionContext>;
//    fn get_return_state(&self, index: isize) -> isize;
//    fn length(&self) -> isize;
//    fn is_empty(&self) -> bool;
//    fn has_empty_path(&self) -> bool;
//    fn hash_code(&self)->i32;
//}

//todo make return states ATNStateRef
#[derive(Eq, Clone, Debug)]
pub enum PredictionContext {
    Singleton(SingletonPredictionContext),
    Array(ArrayPredictionContext),
//    Empty {
//        cached_hash: i32,
//    },
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ArrayPredictionContext {
    cached_hash: i32,
    //todo maybe Rc here too?
    parents: Vec<Option<Box<PredictionContext>>>,
    return_states: Vec<isize>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SingletonPredictionContext {
    cached_hash: i32,
    parent_ctx: Option<Box<PredictionContext>>,
    return_state: isize,
}

impl SingletonPredictionContext {
    fn is_empty(&self) -> bool {
        self.return_state == PREDICTION_CONTEXT_EMPTY_RETURN_STATE
            && self.parent_ctx == None
    }
}

impl PartialEq for PredictionContext {
    fn eq(&self, other: &Self) -> bool {
        self.hash_code() == other.hash_code()
    }
}

impl Hash for PredictionContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.hash_code())
    }
}

lazy_static! {
    pub static ref EMPTY_PREDICTION_CONTEXT: PredictionContext =
        PredictionContext::new_empty();
}

impl PredictionContext {
//    pub fn size(ctx:Option<&PredictionContext>) -> usize{
//        if ctx.is_none() {return 0;}
//
//        match ctx.unwrap() {
//            PredictionContext::Singleton(x) => return 1+Self::size(x.parent_ctx.as_deref()),
//            PredictionContext::Array(x) =>
//                return x.parents.len() + x.parents.iter()
//                    .map(|x|Self::size(x.as_deref())).sum::<usize>(),
//        }
//    }

    pub fn new(cached_hash: isize) -> PredictionContext {
        unimplemented!()
    }

    pub fn new_array(
        parents: Vec<Option<Box<PredictionContext>>>,
        return_states: Vec<isize>,
    ) -> PredictionContext {
        let mut ctx = PredictionContext::Array(ArrayPredictionContext {
            cached_hash: 0,
            parents,
            return_states,
        });
        ctx.calc_hash();
        ctx
    }

    pub fn new_singleton(
        parent_ctx: Option<Box<PredictionContext>>,
        return_state: isize,
    ) -> PredictionContext {
        let mut ctx = PredictionContext::Singleton(SingletonPredictionContext {
            cached_hash: 0,
            parent_ctx,
            return_state,
        });
        ctx.calc_hash();
        ctx
    }

    pub fn new_empty() -> PredictionContext {
        let mut ctx = PredictionContext::Singleton(SingletonPredictionContext {
            cached_hash: 0,
            parent_ctx: None,
            return_state: PREDICTION_CONTEXT_EMPTY_RETURN_STATE,
        });
        ctx.calc_hash();
        ctx
    }

    pub fn calc_hash(&mut self) {
        let mut hasher = MurmurHasher::default();
        match self {
            PredictionContext::Singleton(SingletonPredictionContext {
                                             parent_ctx,
                                             return_state,
                                             ..
                                         }) => {
                hasher.write_i32(match parent_ctx {
                    None => { 0 }
                    Some(x) => { x.hash_code() }
                });
                hasher.write_i32(*return_state as i32);
            }
            PredictionContext::Array(ArrayPredictionContext {
                                         parents,
                                         return_states,
                                         ..
                                     }) => {
                parents.iter()
                    .for_each(|x| hasher.write_i32(match x {
                        None => { 0 }
                        Some(x) => { x.hash_code() }
                    }));
                return_states.iter()
                    .for_each(|x| hasher.write_i32(*x as i32));
            }
//            PredictionContext::Empty { .. } => {}
        };

        let hash = hasher.finish() as i32;

        match self {
            PredictionContext::Singleton(SingletonPredictionContext { cached_hash, .. })
            | PredictionContext::Array(ArrayPredictionContext { cached_hash, .. })
//            | PredictionContext::Empty { cached_hash, .. }
            => *cached_hash = hash,
        };
    }
    //}
    //
    //impl PredictionContext for BasePredictionContext{
    pub fn take_parent(&mut self, index: usize) -> Option<PredictionContext> {
        match self {
            PredictionContext::Singleton(singleton) => {
                assert_eq!(index, 0);
                singleton.parent_ctx.take()
            }
            PredictionContext::Array(array) => {
                array.parents[index].take()
            }
        }.map(|x| *x)
    }

    pub fn get_return_state(&self, index: usize) -> isize {
        match self {
            PredictionContext::Singleton(SingletonPredictionContext { return_state, .. }) => {
                assert_eq!(index, 0);
                *return_state
            }
            PredictionContext::Array(ArrayPredictionContext { return_states, .. }) => return_states[index],
//            PredictionContext::Empty { .. } => {
//                assert_eq!(index, 0);
//                base_prediction_context_empty_return_state
//            }
        }
    }

    pub fn length(&self) -> usize {
        match self {
            PredictionContext::Singleton { .. } => 1,
            PredictionContext::Array(ArrayPredictionContext { return_states, .. }) => return_states.len(),
//            PredictionContext::Empty { .. } =.> 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let PredictionContext::Singleton(
            singleton
        ) = self {
            return singleton.is_empty();
        }
        self.get_return_state(0) == PREDICTION_CONTEXT_EMPTY_RETURN_STATE
    }

    pub fn has_empty_path(&self) -> bool {
        self.get_return_state(self.length() - 1) == PREDICTION_CONTEXT_EMPTY_RETURN_STATE
    }

    pub fn hash_code(&self) -> i32 {
        match self {
            PredictionContext::Singleton(SingletonPredictionContext { cached_hash, .. })
            | PredictionContext::Array(ArrayPredictionContext { cached_hash, .. })
//            | PredictionContext::Empty { cached_hash, .. }
            => *cached_hash,
        }
    }

//    pub fn is_consistent(&self) -> bool {
//        match self {
//            Singleton(x) => {x.parent_ctx.is_some() || x.return_state == PREDICTION_CONTEXT_EMPTY_RETURN_STATE},
//            Array(ArrayPredictionContext{ cached_hash, parents, return_states }) => {
//                parents.iter().zip(return_states.iter())
//                    .all(|(a,&b)|
//                        (a.is_some() && a.as_deref().unwrap().is_consistent()) || b == PREDICTION_CONTEXT_EMPTY_RETURN_STATE)
//            },
//        }
//    }

    fn into_array(self) -> ArrayPredictionContext {
        match self {
            PredictionContext::Singleton(s) => {
                ArrayPredictionContext {
                    cached_hash: 0,
                    parents: vec![s.parent_ctx],
                    return_states: vec![s.return_state],
                }
            }
            PredictionContext::Array(arr) => { arr }
        }
    }

    pub fn merge(a: PredictionContext, b: PredictionContext, root_is_wildcard: bool) -> PredictionContext {
        if a == b { return a; }

        let r = match (a, b) {
            (PredictionContext::Singleton(a), PredictionContext::Singleton(b)) => {
                Self::merge_singletons(a, b, root_is_wildcard)
            }
            (a, b) => {
                if root_is_wildcard {
                    if a.is_empty() { return Self::new_empty(); }
                    if b.is_empty() { return Self::new_empty(); }
                }

                Self::merge_arrays(a.into_array(), b.into_array(), root_is_wildcard)
            }
        };
        return r;
    }

    fn merge_singletons(mut a: SingletonPredictionContext, mut b: SingletonPredictionContext, root_is_wildcard: bool/*, mergeCache: * DoubleDict*/) -> PredictionContext {
        Self::merge_root(&mut a, &mut b, root_is_wildcard).unwrap_or_else(||
            if a.return_state == b.return_state {
                let parent = Self::merge(*a.parent_ctx.clone().unwrap(), *b.parent_ctx.clone().unwrap(), root_is_wildcard);
                if Some(&parent) == a.parent_ctx.as_deref() { return Singleton(a); }
                if Some(&parent) == b.parent_ctx.as_deref() { return Singleton(b); }
                Self::new_singleton(Some(Box::new(parent)), a.return_state)
            } else {
                let mut result = ArrayPredictionContext {
                    cached_hash: -1,
                    parents: vec![a.parent_ctx, b.parent_ctx],
                    return_states: vec![a.return_state, b.return_state],
                };
                if !result.return_states.is_sorted() {
                    result.parents.swap(0, 1);
                    result.return_states.swap(0, 1);
                }
                Array(result)
            }
        )
    }

    fn merge_root(a: &mut SingletonPredictionContext, b: &mut SingletonPredictionContext, root_is_wildcard: bool) -> Option<PredictionContext> {
        if root_is_wildcard {
            if a.is_empty() || b.is_empty() { return Some(Self::new_empty()); }
        } else {
            if a.is_empty() && b.is_empty() { return Some(Self::new_empty()); }
            if a.is_empty() {
                return Some(Self::new_array(
                    vec![b.parent_ctx.take(), None],
                    vec![b.return_state, PREDICTION_CONTEXT_EMPTY_RETURN_STATE],
                ));
            }
            if b.is_empty() {
                return Some(Self::new_array(
                    vec![a.parent_ctx.take(), None],
                    vec![a.return_state, PREDICTION_CONTEXT_EMPTY_RETURN_STATE],
                ));
            }
        }

        None
    }

    fn merge_arrays(mut a: ArrayPredictionContext, mut b: ArrayPredictionContext, root_is_wildcard: bool/*, mergeCache: * DoubleDict*/) -> PredictionContext {
        let mut merged = ArrayPredictionContext {
            cached_hash: -1,
            parents: Vec::with_capacity(a.return_states.len() + b.return_states.len()),
            return_states: Vec::with_capacity(a.return_states.len() + b.return_states.len()),
        };
        let mut i = 0;
        let mut j = 0;

        while i < a.parents.len() && j < b.parents.len() {
            let a_parent = &mut a.parents[i];
            let b_parent = &mut b.parents[j];
            if a.return_states[i] == b.return_states[j] {
                let payload = a.return_states[i];
                let both = payload == PREDICTION_CONTEXT_EMPTY_RETURN_STATE
                    && a_parent.is_none() && b_parent.is_none();
                let ax_ax = a_parent.is_some() && b_parent.is_some()
                    && a_parent == b_parent;

                if both || ax_ax {
                    merged.return_states.push(payload);
                    merged.parents.push(a_parent.take());
                } else {
                    let merged_parent = Self::merge(*a_parent.take().unwrap(), *b_parent.take().unwrap(), root_is_wildcard);
                    merged.return_states.push(payload);
                    merged.parents.push(Some(Box::new(merged_parent)));
                }
                i += 1;
                j += 1;
            } else if a.return_states[i] < b.return_states[j] {
                merged.return_states.push(a.return_states[i]);
                merged.parents.push(a_parent.take());
                i += 1;
            } else {
                merged.return_states.push(b.return_states[j]);
                merged.parents.push(b_parent.take());
                j += 1;
            }
        }

        if i < a.return_states.len() {
            for p in i..a.return_states.len() {
                merged.parents.push(a.parents[p].take());
                merged.return_states.push(a.return_states[p]);
            }
        }
        if j < b.return_states.len() {
            for p in j..b.return_states.len() {
                merged.parents.push(b.parents[p].take());
                merged.return_states.push(b.return_states[p]);
            }
        }

        if merged.parents.len() < a.return_states.len() + b.return_states.len() {
            if merged.parents.len() == 1 {
                return Self::new_singleton(merged.parents[0].take(), merged.return_states[0]);
            }
            merged.return_states.shrink_to_fit();
            merged.parents.shrink_to_fit();
        }


        PredictionContext::combine_common_parents(&mut merged);

        return Array(merged);
    }

    pub fn from_rule_context(atn: &ATN, outer_context: &dyn ParserRuleContext) -> PredictionContext {
        if outer_context.peek_parent().is_none() || ptr::eq(outer_context, empty_ctx().as_ref()) {
            return PredictionContext::new_empty()
        }

        let parent = PredictionContext::from_rule_context(atn, outer_context.peek_parent().unwrap().deref());

        let transition = atn.states[outer_context.get_invoking_state() as usize]
            .get_transitions()
            .first().unwrap()
            .deref()
            .cast::<RuleTransition>();

        PredictionContext::new_singleton(Some(Box::new(parent)), transition.follow_state as isize)
    }

    fn combine_common_parents(array: &mut ArrayPredictionContext) {
        //todo, when PredictionContext will be in arena
    }
}


//
//    fn get_cached_base_prediction_context(context PredictionContext, contextCache: * PredictionContextCache, visited: map[PredictionContext]PredictionContext) -> PredictionContext { unimplemented!() }

pub struct PredictionContextCache {
    cache: HashMap<PredictionContext, PredictionContext, MurmurHasherBuilder>,
}

//
pub struct MurmurHasherBuilder {}

impl BuildHasher for MurmurHasherBuilder {
    type Hasher = MurmurHasher;

    fn build_hasher(&self) -> Self::Hasher {
        MurmurHasher::default()
    }
}

impl PredictionContextCache {
    pub fn new() -> PredictionContextCache {
        PredictionContextCache {
            cache: HashMap::with_hasher(MurmurHasherBuilder {}),
        }
    }

    fn add(&self, _ctx: Box<PredictionContext>) -> &PredictionContext {
        unimplemented!()
    }

    fn get(&self, _ctx: Box<PredictionContext>) -> &PredictionContext {
        unimplemented!()
    }

    fn length(&self) -> isize {
        unimplemented!()
    }
}
