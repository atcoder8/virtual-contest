use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;
use segment_tree::{Monoid, SegmentTree};
use unique_ordering::UniqueOrdering;

const MAX: usize = 10_usize.pow(9);

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        mut shapes: [[usize; 3]; n],
    }

    shapes.iter_mut().for_each(|shape| shape.sort_unstable());
    shapes.sort_unstable_by_key(|shape| (shape[0], Reverse(shape[1])));

    let mut uo = UniqueOrdering::from(shapes.iter().map(|shape| shape[1]).collect_vec());

    let mut st = SegmentTree::<S>::new(n);
    for shape in &shapes {
        let w = uo.position(&shape[1]);

        if st.prod(0..w).0 < shape[2] {
            return true;
        }

        let min_d = st.get(w).0.min(shape[2]);
        st.set(w, S(min_d));
    }

    false
}

#[derive(Debug, Clone, Copy)]
struct S(usize);

impl Monoid for S {
    fn id() -> Self {
        Self(MAX + 1)
    }

    fn prod(&self, rhs: &Self) -> Self {
        Self(self.0.min(rhs.0))
    }
}

pub mod segment_tree {
    //! Performs the following operations on a number sequence of length `n`
    //! consisting of elements of monoid in logarithmic time of `n`.
    //! * Updates the specified element
    //! * Calculates the product of elements in the specified range.

    use std::ops::RangeBounds;

    /// Defines the method signature of the monoid.
    pub trait Monoid: Clone {
        /// The identity element
        fn id() -> Self;

        /// The binary operation
        fn prod(&self, rhs: &Self) -> Self;
    }

    /// # Examples
    ///
    /// ```
    /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
    /// #
    /// #[derive(Debug, Clone, PartialEq)]
    /// struct Data(i32);
    ///
    /// impl Monoid for Data {
    ///     fn id() -> Self {
    ///         Data(0)
    ///     }
    ///
    ///     fn prod(&self, rhs: &Self) -> Self {
    ///         Data(self.0.max(rhs.0))
    ///     }
    /// }
    ///
    /// let seq = vec![Data(3), Data(-1), Data(4), Data(1), Data(-5), Data(9)];
    /// let segtree = SegmentTree::from(seq);
    /// assert_eq!(segtree.prod(1..5), Data(4));
    /// ```
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SegmentTree<M>
    where
        M: Monoid,
    {
        /// Length of sequence
        n: usize,

        /// Sequences and intermediate data
        data: Vec<M>,
    }

    impl<M: Monoid> From<Vec<M>> for SegmentTree<M> {
        fn from(seq: Vec<M>) -> Self {
            let mut segtree = Self::new(seq.len());
            for (i, x) in seq.into_iter().enumerate() {
                segtree.set(i, x);
            }
            segtree
        }
    }

    impl<M: Monoid> SegmentTree<M> {
        /// Creates a Segment Tree for a sequence of length `n`.
        /// All elements of the sequence are initialized with the identity element.
        ///
        /// # Arguments
        ///
        /// * `n` - Length of sequence
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
        ///     }
        /// }
        ///
        /// let segtree = SegmentTree::<Data>::new(5);
        /// assert_eq!(segtree.get(2), Data(0));
        /// ```
        pub fn new(n: usize) -> Self {
            let mut data_len = 1;
            while data_len < n {
                data_len *= 2;
            }
            data_len *= 2;

            Self {
                n,
                data: vec![M::id(); data_len],
            }
        }

        /// Update the `p`-th number in the sequence to `x`.
        ///
        /// # Arguments
        ///
        /// * `p` - Index of the element to update
        /// * `x` - Value to assign
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4)];
        /// let mut segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.get(1), Data(-1));
        ///
        /// segtree.set(1, Data(10));
        /// assert_eq!(segtree.get(1), Data(10));
        /// ```
        pub fn set(&mut self, p: usize, x: M) {
            assert!(
                p < self.n,
                "The specified index {} is outside the range of the sequence; the length of the sequence is {}.",
                p,
                self.n,
            );

            let mut p = p + self.data.len() / 2;
            self.data[p] = x;
            while p != 1 {
                p >>= 1;
                self.data[p] = self.data[2 * p].prod(&self.data[2 * p + 1]);
            }
        }

        /// Returns the `p`-th element.
        ///
        /// # Arguments
        ///
        /// * `p` - Index of the element to get
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4)];
        /// let segtree = SegmentTree::from(seq);
        /// assert_eq!(segtree.get(1), Data(-1));
        /// ```
        pub fn get(&self, p: usize) -> M {
            assert!(
                p < self.n,
                "The specified index {} is outside the range of the sequence; the length of the sequence is {}.",
                p,
                self.n,
            );

            self.data[self.data.len() / 2 + p].clone()
        }

        /// Calculates the product of elements of the sequence in the specified range.
        ///
        /// # Arguments
        ///
        /// * `rng` - Range of the product
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4), Data(1), Data(-5), Data(9)];
        /// let segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.prod(..), Data(9));
        /// assert_eq!(segtree.prod(2..), Data(9));
        /// assert_eq!(segtree.prod(..5), Data(4));
        /// assert_eq!(segtree.prod(2..5), Data(4));
        /// assert_eq!(segtree.prod(2..2), Data(0));
        /// ```
        pub fn prod<R>(&self, rng: R) -> M
        where
            R: RangeBounds<usize>,
        {
            let l = match rng.start_bound() {
                std::ops::Bound::Included(&start_bound) => start_bound,
                std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let r = match rng.end_bound() {
                std::ops::Bound::Included(&end_bound) => end_bound + 1,
                std::ops::Bound::Excluded(&end_bound) => end_bound,
                std::ops::Bound::Unbounded => self.n,
            };

            assert!(l <= r, "The slice index starts at {} but ends at {}", l, r);
            assert!(
                r <= self.n,
                "The specified range {}..{} is outside the range of the sequence; the length of sequence is {}",
                l,
                r,
                self.n,
            );

            let mut sml = M::id();
            let mut smr = M::id();

            let mut l = l + self.data.len() / 2;
            let mut r = r + self.data.len() / 2;

            while l < r {
                if l & 1 != 0 {
                    sml = sml.prod(&self.data[l]);
                    l += 1;
                }

                if r & 1 != 0 {
                    r -= 1;
                    smr = self.data[r].prod(&smr);
                }

                l >>= 1;
                r >>= 1;
            }

            sml.prod(&smr)
        }

        /// Returns the product of all elements of a sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(i32);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(-1), Data(4), Data(1), Data(-5)];
        /// let segtree = SegmentTree::from(seq);
        /// assert_eq!(segtree.all_prod(), Data(4));
        /// ```
        pub fn all_prod(&self) -> M {
            self.data[1].clone()
        }

        /// Performs a binary search on the segment tree.
        ///
        /// Returns one `r` satisfying
        /// `f(op(seq[l], seq[l + 1], ... , seq[r - 1])) == true` and
        /// `f(op(seq[l], seq[l + 1], ... , seq[r])) == false`.
        ///
        /// If no such `r` exists, returns the maximum index of the sequence plus 1;
        /// if the length of the sequence is `n`, then `n` is returned.
        ///
        /// # Arguments
        ///
        /// * `l` - Left boundary of the range of the sequence
        /// * `f` - Mapping from any element of a monoid to a bool value
        ///
        /// # Panics
        ///
        /// Panic if any of the following:
        /// * The identity element is mapped to `false` by `f`.
        /// * The left boundary `l` is outside the range of the sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(usize);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(1), Data(4), Data(1), Data(5), Data(9)];
        /// let segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.bs_right_boundary(1, |&Data(x)| x < 1), 1);
        /// assert_eq!(segtree.bs_right_boundary(1, |&Data(x)| x < 5), 4);
        /// assert_eq!(segtree.bs_right_boundary(1, |&Data(x)| x < 10), 6);
        /// ```
        pub fn bs_right_boundary<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&M) -> bool,
        {
            assert!(
                f(&M::id()),
                "The identity element must be mapped to true by `f`."
            );

            assert!(
                l <= self.n,
                "Left boundary {} is outside the range of the sequence; the length of sequence is {}.",
                l,
                self.n,
            );

            if l == self.n {
                return self.n;
            }

            let size = self.data.len() / 2;

            let mut l = l + size;
            let mut sm = M::id();

            loop {
                while l % 2 == 0 {
                    l >>= 1;
                }

                if !f(&sm.prod(&self.data[l])) {
                    while l < size {
                        l *= 2;
                        let res = sm.prod(&self.data[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }

                    return l - size;
                }

                sm = sm.prod(&self.data[l]);
                l += 1;

                if l & l.wrapping_neg() == l {
                    break;
                }
            }

            self.n
        }

        /// Performs a binary search on the segment tree.
        ///
        /// Returns one `l` satisfying
        /// `f(op(seq[l], seq[l + 1], ... , seq[r - 1])) == true` and
        /// `f(op(seq[l - 1], seq[l + 1], ... , seq[r - 1])) == false`.
        ///
        /// If no such `l` exists, returns `0`.
        ///
        /// # Arguments
        ///
        /// * `r` - Right boundary of the range of the sequence
        /// * `f` - Mapping from any element of a monoid to a bool value
        ///
        /// # Panics
        ///
        /// Panic if any of the following:
        /// * The identity element is mapped to `false` by `f`.
        /// * The right boundary `r` is outside the range of the sequence.
        ///
        /// # Examples
        ///
        /// ```
        /// # use atcoder8_library::segment_tree::{Monoid, SegmentTree};
        /// #
        /// #[derive(Debug, Clone, PartialEq)]
        /// struct Data(usize);
        ///
        /// impl Monoid for Data {
        ///     fn id() -> Self {
        ///         Data(0)
        ///     }
        ///
        ///     fn prod(&self, rhs: &Self) -> Self {
        ///         Data(self.0.max(rhs.0))
        ///     }
        /// }
        ///
        /// let seq = vec![Data(3), Data(1), Data(4), Data(1), Data(5), Data(9)];
        /// let segtree = SegmentTree::from(seq);
        ///
        /// assert_eq!(segtree.bs_left_boundary(4, |&Data(x)| x < 1), 4);
        /// assert_eq!(segtree.bs_left_boundary(4, |&Data(x)| x < 3), 3);
        /// assert_eq!(segtree.bs_left_boundary(4, |&Data(x)| x < 5), 0);
        /// ```
        pub fn bs_left_boundary<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&M) -> bool,
        {
            assert!(
                f(&M::id()),
                "The identity element must be mapped to true by `f`."
            );

            assert!(
                r <= self.n,
                "Right boundary {} is outside the range of the sequence; the length of sequence is {}.",
                r,
                self.n,
            );

            if r == 0 {
                return 0;
            }

            let size = self.data.len() / 2;

            let mut r = r + size;
            let mut sm = M::id();

            loop {
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }

                if !f(&self.data[r].prod(&sm)) {
                    while r < size {
                        r = 2 * r + 1;
                        let res = self.data[r].prod(&sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }

                    return r + 1 - size;
                }

                sm = self.data[r].prod(&sm);

                if r & r.wrapping_neg() == r {
                    break;
                }
            }

            0
        }
    }
}

pub mod unique_ordering {
    //! Module for ordering unique elements.

    use std::ops::Index;

    /// Structure for ordering unique elements.
    #[derive(Debug, Clone)]
    pub struct UniqueOrdering<T> {
        /// A sequence containing the elements to be ordered.
        seq: Vec<T>,

        /// A flag indicating whether `seq` is sorted and deduplicated.
        organized: bool,
    }

    impl<T> Default for UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure for ordering unique elements.
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> From<Vec<T>> for UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure by initializing the elements to be ordered with `seq`.
        fn from(seq: Vec<T>) -> Self {
            Self {
                seq,
                organized: false,
            }
        }
    }

    impl<T> Index<usize> for UniqueOrdering<T> {
        type Output = T;

        /// Returns the `index`-th (0-based) unique element.
        fn index(&self, index: usize) -> &Self::Output {
            &self.seq[index]
        }
    }

    impl<T> UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure for ordering unique elements.
        pub fn new() -> Self {
            Self {
                seq: vec![],
                organized: true,
            }
        }

        /// Adds the elements to be ordered.
        pub fn push(&mut self, x: T) {
            self.seq.push(x);
            self.organized = false;
        }

        /// Appends all elements of the iterator to the elements to be ordered.
        pub fn extend<I>(&mut self, other: I)
        where
            I: IntoIterator<Item = T>,
        {
            self.seq.extend(other);
            self.organized = false;
        }

        /// Sorts the sequence of stored elements in ascending order and removes all duplicates.
        fn organize(&mut self) {
            if !self.organized {
                self.seq.sort_unstable();
                self.seq.dedup();
                self.organized = true;
            }
        }

        /// Returns the `x` position of the unique elements sorted in ascending order.
        pub fn position(&mut self, x: &T) -> usize {
            self.organize();

            self.seq.binary_search(x).unwrap_or_else(|_| {
                panic!("The position of `x` is undefined.");
            })
        }

        /// Returns the `index`-th (0-based) unique element.
        ///
        /// Returns `None` if the `index`-th element does not exist.
        pub fn get(&mut self, index: usize) -> Option<&T> {
            self.seq.get(index)
        }

        /// Returns the number of unique elements.
        pub fn unique_len(&mut self) -> usize {
            self.organize();

            self.seq.len()
        }
    }
}
