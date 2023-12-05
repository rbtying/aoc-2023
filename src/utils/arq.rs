//! Dynamic ARQ tree, borrowed from rust-algorithms (source:
//! https://github.com/EbTech/rust-algorithms/blob/master/src/range_query/)

use crate::prelude::*;

pub trait ArqSpec {
    /// Type of underlying array elements.
    type S: Clone;
    /// Type of data representing an endomorphism.
    // Note that while a Fn(S) -> S may seem like a more natural representation
    // for an endomorphism, compositions would then have to delegate to each of
    // their parts. This representation is more efficient.
    type F: Clone;

    /// Must satisfy the Associative Law:
    /// For all a,b,c, op(a, op(b, c)) = op(op(a, b), c)
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    /// Must satisfy the Identity Law:
    /// For all a, op(a, identity()) = op(identity(), a) = a
    fn identity() -> Self::S;
    /// Must satisfy the Composition Law:
    /// For all f,g,a, apply(compose(f, g), a) = apply(f, apply(g, a))
    fn compose(f: &Self::F, g: &Self::F) -> Self::F;
    /// Must satisfy the Distributive Law:
    /// For all f,a,b, apply(f, op(a, b), s+t) = op(apply(f, a, s), apply(f, b, t))
    /// The `size` parameter makes this law easier to satisfy in certain cases.
    fn apply(f: &Self::F, a: &Self::S, size: isize) -> Self::S;

    // The following relaxations to the laws may apply.
    // If only point updates are made, the Composition and Distributive Laws
    // no longer apply.
    // - compose() is never called, so it can be left unimplemented!().
    // - apply() is only ever called on leaves, i.e., with size == 1.
    // If only point queries are made, the Associative and Distributive Laws
    // no longer apply.
    // - op()'s result only matters when identity() is an argument.
    // - apply()'s result only matters on leaves, i.e., with size == 1.
}

/// Range Minimum Query (RMQ), a classic application of ARQ.
/// update(l, r, &f) sets all entries a[l..=r] to f.
/// query(l, r) finds the minimum value in a[l..=r].
//
// Exercises: try augmenting this struct to find the index of a minimum element
// in a range query, as well as the number of elements equal to the minimum.
// Then instead of overwriting values with a constant assignment a[i] = f,
// try supporting addition: a[i] += f.
pub enum AssignMin {}
impl ArqSpec for AssignMin {
    type S = isize;
    type F = isize;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.min(b)
    }
    fn identity() -> Self::S {
        isize::max_value()
    }
    fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
        f
    }
    fn apply(&f: &Self::F, _: &Self::S, _: isize) -> Self::S {
        f
    }
}

pub enum OverlappingIntervals {}
impl ArqSpec for OverlappingIntervals {
    // Each array item "contains" all of the ranges that overlap it.
    // This is implemented by inserting each interval by `update`-ing the
    // interval with the value.
    type S = HashSet<usize>;

    // The return value of aggregating over all of values in the range is the
    // unique values in that range.
    type F = HashSet<usize>;

    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        a.union(b).copied().collect()
    }
    fn identity() -> Self::S {
        HashSet::default()
    }
    fn compose(f: &Self::F, f2: &Self::F) -> Self::F {
        f.union(f2).copied().collect()
    }
    fn apply(f: &Self::F, s: &Self::S, _: isize) -> Self::S {
        f.union(s).copied().collect()
    }
}

/// Range Sum Query, a slightly trickier classic application of ARQ.
/// update(l, r, &f) sets all entries a[l..=r] to f.
/// query(l, r) sums all the entries a[l..=r].
///
/// # Panics
///
/// Associated functions will panic on overflow.
//
// Note that while the `size` parameter seems necessary to satisfy the
// Distributive Law, it is merely a convenience: in essence what we've done
// is move to the product monoid of tuples (value, size_of_subtree).
//
// In mathematical jargon, we say that constant assignment f(a) = f is not an
// endomorphism on (isize, +) because f(a+b) = f != 2*f = f(a) + f(b).
// On the other hand, f((a, s)) = (f*s, s) is indeed an endomorphism on pairs
// with vector addition: f((a, s) + (b, t)) = f((a+b, s+t)) = (f*(s+t), s+t)
//                       = (f*s, s) + (f*t, t) = f((a,s)) + f((b,t)).
pub enum AssignSum {}
impl ArqSpec for AssignSum {
    type S = isize;
    type F = isize;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }
    fn identity() -> Self::S {
        0
    }
    fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
        f
    }
    fn apply(&f: &Self::F, _: &Self::S, size: isize) -> Self::S {
        f * size
    }
}

/// Supply & Demand, based on https://codeforces.com/gym/102218/problem/F
/// update(i, i, &(p, o)) increases supply by p and demand by o at time i.
/// query(l, r) computes total supply and demand at times l to r, as well as
//              how much of the supply is subsequently met by the demand.
//
// Note that the apply() operation is only correct when applied to leaf nodes.
// Therefore, update() must only be used in "eager" mode, i.e., with l == r.
// compose() should be unimplemented!() to prevent accidental "lazy" updates.
pub enum SupplyDemand {}
impl ArqSpec for SupplyDemand {
    type S = (isize, isize, isize); // production, orders, sales
    type F = (isize, isize);
    fn op((p1, o1, s1): &Self::S, (p2, o2, s2): &Self::S) -> Self::S {
        let extra = (p1 - s1).min(o2 - s2);
        (p1 + p2, o1 + o2, s1 + s2 + extra)
    }
    fn identity() -> Self::S {
        (0, 0, 0)
    }
    fn compose(_: &Self::F, _: &Self::F) -> Self::F {
        unimplemented!()
    }
    fn apply(&(p_add, o_add): &Self::F, &(p, o, _): &Self::S, s: isize) -> Self::S {
        assert_eq!(s, 1);
        let p = p + p_add;
        let o = o + o_add;
        (p, o, p.min(o))
    }
}

pub struct DynamicArqNode<T: ArqSpec> {
    val: T::S,
    app: Option<T::F>,
    down: (usize, usize),
}

// TODO: in a future Rust version, this might be replaced by a #[derive(Clone)]
impl<T: ArqSpec> Clone for DynamicArqNode<T> {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            app: self.app.clone(),
            down: self.down,
        }
    }
}

impl<T: ArqSpec> Default for DynamicArqNode<T> {
    fn default() -> Self {
        Self {
            val: T::identity(),
            app: None,
            down: (usize::max_value(), usize::max_value()),
        }
    }
}

impl<T: ArqSpec> DynamicArqNode<T> {
    fn apply(&mut self, f: &T::F, size: isize) {
        self.val = T::apply(f, &self.val, size);
        if size > 1 {
            let h = match self.app {
                Some(ref g) => T::compose(f, g),
                None => f.clone(),
            };
            self.app = Some(h);
        }
    }
}

pub type ArqView = (usize, isize);

/// A dynamic, and optionally persistent, associative range query data structure.
pub struct DynamicArq<T: ArqSpec> {
    nodes: Vec<DynamicArqNode<T>>,
    is_persistent: bool,
}

impl<T: ArqSpec> DynamicArq<T> {
    /// Initializes the data structure without creating any nodes.
    pub fn new(is_persistent: bool) -> Self {
        Self {
            nodes: vec![],
            is_persistent,
        }
    }

    /// Lazily builds a tree initialized to the identity.
    pub fn build_from_identity(&mut self, size: isize) -> ArqView {
        self.nodes.push(DynamicArqNode::default());
        (self.nodes.len() - 1, size)
    }

    /// Builds a tree whose leaves are set to a given non-empty slice.
    pub fn build_from_slice(&mut self, init_val: &[T::S]) -> ArqView {
        if init_val.len() == 1 {
            let root = DynamicArqNode {
                val: init_val[0].clone(),
                ..Default::default()
            };
            self.nodes.push(root);
            (self.nodes.len() - 1, 1)
        } else {
            let ls = init_val.len() / 2;
            let (l_init, r_init) = init_val.split_at(ls);
            let l_view = self.build_from_slice(l_init);
            let r_view = self.build_from_slice(r_init);
            self.merge_equal_sized(l_view, r_view)
        }
    }

    /// Merges two balanced subtrees into a single tree with a 0-indexed view.
    pub fn merge_equal_sized(&mut self, (lp, ls): ArqView, (rp, rs): ArqView) -> ArqView {
        assert!(ls == rs || ls + 1 == rs);
        let p = self.nodes.len();
        let root = DynamicArqNode {
            down: (lp, rp),
            ..Default::default()
        };
        self.nodes.push(root);
        self.pull(p);
        (p, ls + rs)
    }

    pub fn push(&mut self, (p, s): ArqView) -> (ArqView, ArqView) {
        if self.nodes[p].down.0 == usize::max_value() {
            self.nodes.push(DynamicArqNode::default());
            self.nodes.push(DynamicArqNode::default());
            self.nodes[p].down = (self.nodes.len() - 2, self.nodes.len() - 1)
        };
        let (lp, rp) = self.nodes[p].down;
        let ls = s / 2;
        if let Some(ref f) = self.nodes[p].app.take() {
            self.nodes[lp].apply(f, ls);
            self.nodes[rp].apply(f, s - ls);
        }
        ((lp, ls), (rp, s - ls))
    }

    pub fn pull(&mut self, p: usize) {
        let (lp, rp) = self.nodes[p].down;
        let left_val = &self.nodes[lp].val;
        let right_val = &self.nodes[rp].val;
        self.nodes[p].val = T::op(left_val, right_val);
    }

    fn clone_node(&mut self, p_orig: usize) -> usize {
        if self.is_persistent {
            let node = self.nodes[p_orig].clone();
            self.nodes.push(node);
            self.nodes.len() - 1
        } else {
            p_orig
        }
    }

    pub fn point_update(&mut self, view: ArqView, idx: isize, f: &T::F) -> ArqView {
        self.update(view, idx, idx, f)
    }

    /// Applies the endomorphism f to all entries from l to r, inclusive.
    /// If l == r, the updates are eager. Otherwise, they are lazy.
    pub fn update(&mut self, view: ArqView, l: isize, r: isize, f: &T::F) -> ArqView {
        let (p_orig, s) = view;
        if r < 0 || s - 1 < l {
            view
        } else if l <= 0 && s - 1 <= r {
            let p_clone = self.clone_node(p_orig);
            self.nodes[p_clone].apply(f, s);
            (p_clone, s)
        } else {
            let (l_view, r_view) = self.push(view);
            let ls = l_view.1;
            let p_clone = self.clone_node(p_orig);
            let lp_clone = self.update(l_view, l, r, f).0;
            let rp_clone = self.update(r_view, l - ls, r - ls, f).0;
            self.nodes[p_clone].down = (lp_clone, rp_clone);
            self.pull(p_clone);
            (p_clone, s)
        }
    }

    /// Returns the aggregate range query on all entries from l to r, inclusive.
    pub fn query(&mut self, view: ArqView, l: isize, r: isize) -> T::S {
        let (p, s) = view;
        if r < 0 || s - 1 < l {
            T::identity()
        } else if l <= 0 && s - 1 <= r {
            self.nodes[p].val.clone()
        } else {
            let (l_view, r_view) = self.push(view);
            let ls = l_view.1;
            let l_agg = self.query(l_view, l, r);
            let r_agg = self.query(r_view, l - ls, r - ls);
            T::op(&l_agg, &r_agg)
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}

/// An example of binary search to find the first position whose element is negative.
/// The DynamicArq version works on trees of any size, not necessarily a power of two.
pub fn first_negative(arq: &mut DynamicArq<AssignMin>, view: ArqView) -> Option<isize> {
    let (p, s) = view;
    if s == 1 {
        Some(0).filter(|_| arq.nodes[p].val < 0)
    } else {
        let (l_view, r_view) = arq.push(view);
        let (lp, ls) = l_view;
        if arq.nodes[lp].val < 0 {
            first_negative(arq, l_view)
        } else {
            first_negative(arq, r_view).map(|x| ls + x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_in_range() {
        let mut arq = DynamicArq::<AssignMin>::new(false);
        let mut view = arq.build_from_identity(std::isize::MAX);

        for (idx, interval) in [0..=4, 3..=8, 2..=6].into_iter().enumerate() {
            let idx = idx as isize;
            view = arq.update(view, *interval.start(), *interval.end(), &idx);
        }
        assert_eq!(arq.query(view, 0, 2), 0);
        assert_eq!(arq.query(view, 2, 3), 2);
        assert_eq!(arq.query(view, 4, 6), 2);
        assert_eq!(arq.query(view, 7, 8), 1);
        assert_eq!(arq.query(view, 10, 12), std::isize::MAX);
    }

    #[test]
    fn test_find_overlapping_intervals() {
        let mut arq = DynamicArq::<OverlappingIntervals>::new(false);
        let mut view = arq.build_from_identity(std::isize::MAX);

        let mut sizes = vec![];
        for (idx, interval) in [0..=4, 3..=8, 2..=6].into_iter().enumerate() {
            view = arq.update(
                view,
                *interval.start(),
                *interval.end(),
                &([idx].into_iter().collect()),
            );
            sizes.push(arq.size());
        }
        assert_eq!(sizes, vec![125, 129, 133]);
        assert_eq!(arq.query(view, 0, 8), [0, 1, 2].into_iter().collect());
        assert_eq!(arq.query(view, 5, 6), [1, 2].into_iter().collect());
        assert_eq!(arq.query(view, 0, 2), [0, 2].into_iter().collect());
    }
}
