use proconio::{
    input,
    marker::{Bytes, Usize1},
};

use crate::atcoder8_library::fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        mut s: Bytes,
        q: usize,
    }

    let mut fts = vec![FenwickTree::<i64>::new(n); 26];
    for (i, &c) in s.iter().enumerate() {
        fts[(c - b'a') as usize].set(i, 1);
    }

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                (i, c): (Usize1, char),
            }

            fts.iter_mut().for_each(|ft| {
                if ft.get(i) == 1 {
                    ft.set(i, 0);
                }
            });
            fts[(c as u8 - b'a') as usize].set(i, 1);
        } else {
            input! {
                (l, r): (Usize1, usize),
            }

            let ans = fts.iter().filter(|ft| ft.sum(l..r) != 0).count();
            println!("{}", ans);
        }
    }
}

pub mod atcoder8_library {
    pub mod fenwick_tree {
        //! Processes the following query in `O(log(n))` time
        //! for a sequence of numbers with `n` elements:
        //! * Update one element
        //! * Calculate the sum of the elements of a range
        //! * Gets the elements of a number sequence.

        use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};

        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..), 11);
        /// ```
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct FenwickTree<T>(Vec<T>)
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T>;

        impl<T> From<Vec<T>> for FenwickTree<T>
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T>,
        {
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 9);
            /// ```
            fn from(t: Vec<T>) -> Self {
                let mut ft = FenwickTree::new(t.len());
                for (i, x) in t.into_iter().enumerate() {
                    ft.add(i, x);
                }
                ft
            }
        }

        impl<T> FenwickTree<T>
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T>,
        {
            /// Constructs a `FenwickTree<T>` with `n` elements.
            ///
            /// Each element is initialized with `T::default()`.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::<i32>::new(5);
            /// assert_eq!(ft.sum(..), 0);
            /// ```
            pub fn new(n: usize) -> Self {
                FenwickTree(vec![T::default(); n])
            }

            /// Add `x` to the `p`-th element.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 9);
            ///
            /// ft.add(3, 100);
            /// assert_eq!(ft.sum(2..6), 109);
            /// ```
            pub fn add(&mut self, p: usize, x: T) {
                let FenwickTree(data) = self;
                let n = data.len();

                assert!(p < n);

                let mut p = p + 1;
                while p <= n {
                    data[p - 1] += x.clone();
                    p += p & p.overflowing_neg().0;
                }
            }

            /// Sets `x` to the `p`-th element.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 9);
            ///
            /// ft.set(3, 100);
            /// assert_eq!(ft.sum(2..6), 108);
            /// ```
            pub fn set(&mut self, p: usize, x: T) {
                let FenwickTree(data) = self;
                let n = data.len();

                assert!(p < n);

                let t = x - self.get(p);
                self.add(p, t);
            }

            /// Compute the sum of the range [0, r).
            fn inner_sum(&self, r: usize) -> T {
                let mut r = r;
                let mut s = T::default();
                while r > 0 {
                    s += self.0[r - 1].clone();
                    r -= r & r.overflowing_neg().0;
                }
                return s;
            }

            /// Calculate the total of the range.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
            /// assert_eq!(ft.sum(..), 13);
            /// assert_eq!(ft.sum(2..), 11);
            /// assert_eq!(ft.sum(..6), 11);
            /// assert_eq!(ft.sum(2..6), 9);
            /// assert_eq!(ft.sum(6..2), 0);
            /// ```
            pub fn sum<R>(&self, rng: R) -> T
            where
                R: RangeBounds<usize>,
            {
                let n = self.0.len();

                let l = match rng.start_bound() {
                    std::ops::Bound::Included(&start_bound) => start_bound,
                    std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                    std::ops::Bound::Unbounded => 0,
                };

                let r = match rng.end_bound() {
                    std::ops::Bound::Included(&end_bound) => end_bound + 1,
                    std::ops::Bound::Excluded(&end_bound) => end_bound,
                    std::ops::Bound::Unbounded => n,
                };

                assert!(l <= n && r <= n);

                if l >= r {
                    T::default()
                } else {
                    self.inner_sum(r) - self.inner_sum(l)
                }
            }

            /// Returns the value of an element in a sequence of numbers.
            /// Calculate the total of the range.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let ft = FenwickTree::from(vec![3, -1, 4, -1, 5]);
            /// assert_eq!(ft.get(2), 4);
            /// ```
            pub fn get(&self, p: usize) -> T {
                assert!(p < self.0.len());

                self.sum(p..=p)
            }
        }

        impl<T> FenwickTree<T>
        where
            T: Clone + Default + Sub<T, Output = T> + AddAssign<T> + SubAssign<T>,
        {
            /// Subtract `x` from the `p`-th element.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::fenwick_tree::FenwickTree;
            ///
            /// let mut ft = FenwickTree::<u32>::from(vec![3, 1, 4, 1, 5, 9, 2]);
            /// assert_eq!(ft.sum(2..6), 19);
            ///
            /// ft.sub(3, 1);
            /// assert_eq!(ft.sum(2..6), 18);
            /// ```
            pub fn sub(&mut self, p: usize, x: T) {
                let FenwickTree(data) = self;
                let n = data.len();

                assert!(p < n);

                let mut p = p + 1;
                while p <= n {
                    data[p - 1] -= x.clone();
                    p += p & p.overflowing_neg().0;
                }
            }
        }
    }
}
