use std::cmp::{max, min, Ordering};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Interval {
    pub a: isize,
    pub b: isize,
}

impl Interval {
    /* stop is not included! */
    fn new_interval(_start: isize, _stop: isize) -> Interval {
        unimplemented!()
    }

    fn contains(&self, _item: isize) -> bool {
        unimplemented!()
    }

    fn String(&self) -> String {
        unimplemented!()
    }

    fn length(&self) -> isize {
        self.b - self.a
    }

    fn union(&self, another: &Interval) -> Interval {
        Interval {
            a: min(self.a, another.a),
            b: max(self.b, another.b),
        }
    }

    /** Does self start completely before other? Disjoint */
    pub fn startsBeforeDisjoint(&self, other: &Interval) -> bool {
        return self.a < other.a && self.b < other.a;
    }

    /** Does self start at or before other? Nondisjoint */
    pub fn startsBeforeNonDisjoint(&self, other: &Interval) -> bool {
        return self.a <= other.a && self.b >= other.a;
    }

    /** Does self.a start after other.b? May or may not be disjoint */
    pub fn startsAfter(&self, other: &Interval) -> bool {
        return self.a > other.a;
    }

    /** Does self start completely after other? Disjoint */
    pub fn startsAfterDisjoint(&self, other: &Interval) -> bool {
        return self.a > other.b;
    }

    /** Does self start after other? NonDisjoint */
    pub fn startsAfterNonDisjoint(&self, other: &Interval) -> bool {
        return self.a > other.a && self.a <= other.b; // self.b>=other.b implied
    }

    /** Are both ranges disjoint? I.e., no overlap? */
    pub fn disjoint(&self, other: &Interval) -> bool {
        return self.startsBeforeDisjoint(other) || self.startsAfterDisjoint(other);
    }

    /** Are two intervals adjacent such as 0..41 and 42..42? */
    pub fn adjacent(&self, other: &Interval) -> bool {
        return self.a == other.b + 1 || self.b == other.a - 1;
    }

    //    public boolean properlyContains(Interval other) {
    //    return other.a >= self.a && other.b <= self.b;
    //    }
    //
    //    /** Return the interval computed from combining self and other */
    //    public Interval union(Interval other) {
    //    return Interval.of(Math.min(a, other.a), Math.max(b, other.b));
    //    }
    //
    //    /** Return the interval in common between self and o */
    //    public Interval intersection(Interval other) {
    //    return Interval.of(Math.max(a, other.a), Math.min(b, other.b));
    //    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IntervalSet {
    intervals: Vec<Interval>,
    read_only: bool,
}

impl IntervalSet {
    pub fn new_interval_set() -> IntervalSet {
        IntervalSet {
            intervals: Vec::new(),
            read_only: false,
        }
    }

    pub fn get_min(&self) -> Option<isize> {
        self.intervals.first().map(|x| x.a)
    }

    pub fn add_one(&mut self, _v: isize) {
        self.add_range(_v, _v)
    }

    pub fn add_range(&mut self, l: isize, h: isize) {
        self.add_interval(Interval { a: l, b: h })
    }

    pub fn add_interval(&mut self, added: Interval) {
        if added.length() < 0 {
            return;
        }

        let mut i = 0;
        while let Some(r) = self.intervals.get_mut(i) {
            if *r == added {
                return;
            }

            if added.adjacent(r) || !added.disjoint(r) {
                // next to each other, make a single larger interval
                let bigger = added.union(r);
                *r = bigger;
                // make sure we didn't just create an interval that
                // should be merged with next interval in list
                loop {
                    i += 1;
                    let next = match self.intervals.get(i) {
                        Some(v) => v,
                        None => break,
                    };
                    if !bigger.adjacent(next) && bigger.disjoint(next) {
                        break;
                    }

                    // if we bump up against or overlap next, merge
                    self.intervals[i - 1] = bigger.union(next); // set to 3 merged ones
                    self.intervals.remove(i);
                }
                return;
            }
            if added.startsBeforeDisjoint(r) {
                // insert before r
                self.intervals.insert(i - 1, added);
                return;
            }
            i += 1;
        }

        self.intervals.push(added);
    }

    fn add_set(&self, _other: &IntervalSet) -> IntervalSet {
        unimplemented!()
    }

    fn complement(&self, _start: isize, _stop: isize) -> IntervalSet {
        unimplemented!()
    }

    pub fn contains(&self, _item: isize) -> bool {
        self.intervals.binary_search_by(|x| {
            if _item < x.a { return Ordering::Greater; }
            if _item > x.b { return Ordering::Less; }
            Ordering::Equal
        }).is_ok()
    }

    fn length(&self) -> isize {
        unimplemented!()
    }

    fn remove_range(&self, _v: &Interval) {
        unimplemented!()
    }

    fn remove_one(&self, _v: isize) {
        unimplemented!()
    }

    fn String(&self) -> String {
        unimplemented!()
    }

    fn String_verbose(
        &self,
        _literalNames: Vec<String>,
        _symbolicNames: Vec<String>,
        _elemsAreChar: bool,
    ) -> String {
        unimplemented!()
    }

    fn to_char_String(&self) -> String {
        unimplemented!()
    }

    fn to_index_String(&self) -> String {
        unimplemented!()
    }

    fn to_token_String(&self, _literalNames: Vec<String>, _symbolicNames: Vec<String>) -> String {
        unimplemented!()
    }

    fn element_name(
        &self,
        _literalNames: Vec<String>,
        _symbolicNames: Vec<String>,
        _a: isize,
    ) -> String {
        unimplemented!()
    }
}

mod test {
    use crate::interval_set::IntervalSet;

    #[test]
    fn test() {
        let mut set = IntervalSet {
            intervals: Vec::new(),
            read_only: false,
        };
        set.add_range(1, 2);
    }
}
