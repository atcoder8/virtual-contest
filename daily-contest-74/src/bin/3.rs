use proconio::{input, marker::Usize1};

use crate::atcoder8_library::{fenwick_tree::FenwickTree, union_find::UnionFind};

fn main() {
    input! {
        n: usize,
        q: usize,
        txyv: [(usize, Usize1, Usize1, i64); q],
    }

    let mut uf = UnionFind::new(n);
    let mut ft: FenwickTree<i64> = FenwickTree::new(n);

    for &(t, x, y, v) in &txyv {
        if t == 0 {
            uf.merge(x, y);

            if x % 2 == 0 {
                ft.set(x, v);
            } else {
                ft.set(x, -v);
            }
        } else {
            if !uf.same(x, y) {
                println!("Ambiguous");
                continue;
            }

            if x == y {
                println!("{}", v);
                continue;
            }

            let ans = if x < y {
                let sign_x = sign(x);
                let sign_y = sign(y - 1);

                sign_y * (ft.sum(x..y) - sign_x * v)
            } else if x > y {
                let sign_x = sign(x - 1);
                let sign_y = sign(y);

                sign_y * (ft.sum(y..x) - sign_x * v)
            } else {
                v
            };
            println!("{}", ans);
        }
    }
}

fn sign(x: usize) -> i64 {
    if x % 2 == 0 {
        1
    } else {
        -1
    }
}

pub mod atcoder8_library {
    pub mod union_find {
        //! Union-Find processes the following queries on undirected graphs.
        //! * Merge two connected components.
        //! * Determine whether two given nodes are in the same connected component.
        //!
        //! To seed up processing, merge optimization using the number of nodes
        //! of the connected components and path compression are performed.
        //!
        //! The time complexity of each query is `O(A(n))`.
        //! where `n` is the number of nodes in the graph and
        //! `A(n)` is the inverse of the Ackermann function.

        /// This is the value that will be associated with each nodes of the graph.
        #[derive(Debug, Clone, Copy)]
        enum ParentOrSize {
            /// It is used for non-representative nodes and stores the parent node.
            Parent(usize),

            /// It is used for the representative node and
            /// stores the number of nodes of the connected component.
            Size(usize),
        }

        /// Union-Find processes the following queries on undirected graphs.
        /// * Merge two connected components.
        /// * Determine whether two given nodes are in the same connected component.
        ///
        /// To seed up processing, merge optimization using the number of nodes
        /// of the connected components and path compression are performed.
        ///
        /// The time complexity of each query is `O(A(n))`.
        /// where `n` is the number of nodes in the graph and
        /// `A(n)` is the inverse of the Ackermann function.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::union_find::UnionFind;
        ///
        /// let mut uf = UnionFind::new(3);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(0, 1);
        /// assert_eq!(uf.same(0, 2), false);
        /// uf.merge(1, 2);
        /// assert_eq!(uf.same(0, 2), true);
        /// ```
        #[derive(Debug, Default, Clone)]
        pub struct UnionFind {
            /// For each node, one of the following is stored.
            /// * The number of nodes of the connected component to which it belongs.
            /// (If it is a representative node of the connected component.)
            /// * Index of the parent node. (Otherwise.)
            parent_or_size: Vec<ParentOrSize>,

            /// Number of connected components.
            group_num: usize,
        }

        impl UnionFind {
            /// Create an undirected graph with `n` nodes and `0` edges.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.same(0, 2), true);
            /// ```
            pub fn new(n: usize) -> Self {
                UnionFind {
                    parent_or_size: vec![ParentOrSize::Size(1); n],
                    group_num: n,
                }
            }

            /// Return the representative node of the connected component containing node `a`.
            ///
            /// At that time, perform path compression on the nodes on the path from node `a` to the representative node.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// uf.merge(1, 2);
            /// assert_eq!(uf.leader(0), 0);
            /// assert_eq!(uf.leader(1), uf.leader(2));
            /// ```
            pub fn leader(&mut self, a: usize) -> usize {
                // If node `a` is a representative node of the connected component, return `a`.
                if let ParentOrSize::Size(_) = self.parent_or_size[a] {
                    return a;
                }

                // Path from node `a` to the representative node.
                let mut path = vec![];

                // Current node.
                let mut current = a;

                // Record the path to the representative node.
                while let ParentOrSize::Parent(parent) = self.parent_or_size[current] {
                    // Add current node to the path.
                    path.push(current);

                    // Move to the parent node.
                    current = parent;
                }

                // The representative node of the connected component.
                let leader = current;

                // Set nodes on the path as direct children of the representative node.
                path.iter().for_each(|&node| {
                    self.parent_or_size[node] = ParentOrSize::Parent(leader);
                });

                // Return the representative node of the connected component.
                leader
            }

            /// Return whether two nodes `a` and `b` are in the same connected component.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.same(0, 2), true);
            /// ```
            pub fn same(&mut self, a: usize, b: usize) -> bool {
                self.leader(a) == self.leader(b)
            }

            /// Merge each connected component containing nodes `a` and `b`.
            ///
            /// Return `true` if different connected components are newly merged.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.same(0, 2), false);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.same(0, 2), true);
            /// ```
            pub fn merge(&mut self, a: usize, b: usize) -> bool {
                // Representative node of the connected component that contains the node `a`.
                let leader_a = self.leader(a);
                // Representative node of the connected component that contains the node `b`.
                let leader_b = self.leader(b);

                // If nodes `a` and `b` are in the same connected component, return `false` without processing.
                if leader_a == leader_b {
                    return false;
                }

                // Number of nodes of the component containing node `a`.
                let component_size_a = self.size(leader_a);

                // Number of nodes of the component containing node `b`.
                let component_size_b = self.size(leader_b);

                // Number of nodes of the merged component.
                let merged_component_size = component_size_a + component_size_b;

                // Set the parent of the representative node of the smaller sized connected component
                // to be the parent of the other connected component.
                if component_size_a <= component_size_b {
                    self.parent_or_size[leader_a] = ParentOrSize::Parent(leader_b);
                    self.parent_or_size[leader_b] = ParentOrSize::Size(merged_component_size);
                } else {
                    self.parent_or_size[leader_b] = ParentOrSize::Parent(leader_a);
                    self.parent_or_size[leader_a] = ParentOrSize::Size(merged_component_size);
                }

                // Decrease the number of connected components by one.
                self.group_num -= 1;

                // Return `true` because different connected components are newly combined.
                true
            }

            /// Return a list of connected components.
            ///
            /// Each connected component consists of indexes of nodes.
            /// The indexes of the nodes in each connected component are arranged in ascending order.
            /// The list of connected components is sorted in ascending order
            /// with respect to the smallest index of the included nodes.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(5);
            /// uf.merge(1, 2);
            /// uf.merge(2, 3);
            /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3], vec![4]]);
            /// ```
            pub fn groups(&mut self) -> Vec<Vec<usize>> {
                // Number of nodes in graph.
                let element_num = self.parent_or_size.len();

                // List of connected components.
                let mut groups: Vec<Vec<usize>> = vec![];
                // Correspondence between the representative node and group index.
                let mut leader_to_idx: Vec<Option<usize>> = vec![None; element_num];

                // Assign each node in the graph to a group.
                for node in 0..element_num {
                    // Representative node of the connected component to which the `node` belongs.
                    let leader = self.leader(node);

                    if let Some(group_idx) = leader_to_idx[leader] {
                        // Assign to an existing group.
                        groups[group_idx].push(node);
                    } else {
                        // Adding a new group.
                        leader_to_idx[leader] = Some(groups.len());
                        groups.push(vec![node]);
                    }
                }

                // Return a list of groups.
                groups
            }

            /// Return the number of nodes in the connected component to which node `a` belongs.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.size(0), 1);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.size(0), 2);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.size(0), 3);
            /// ```
            pub fn size(&mut self, a: usize) -> usize {
                let leader = self.leader(a);

                match self.parent_or_size[leader] {
                    ParentOrSize::Parent(_) => panic!(),
                    ParentOrSize::Size(size) => size,
                }
            }

            /// Add a new node with degree `0`.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(4);
            /// uf.merge(1, 2);
            /// uf.merge(2, 3);
            /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3]]);
            /// uf.add();
            /// assert_eq!(uf.groups(), vec![vec![0], vec![1, 2, 3], vec![4]]);
            /// ```
            pub fn add(&mut self) {
                self.parent_or_size.push(ParentOrSize::Size(1));
                self.group_num += 1;
            }

            /// Return the number of connected components.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(3);
            /// assert_eq!(uf.group_num(), 3);
            /// uf.merge(0, 1);
            /// assert_eq!(uf.group_num(), 2);
            /// uf.merge(2, 1);
            /// assert_eq!(uf.group_num(), 1);
            /// ```
            pub fn group_num(&self) -> usize {
                self.group_num
            }

            /// Return the number of nodes in the graph.
            ///
            /// # Examples
            ///
            /// ```
            /// use atcoder8_library::union_find::UnionFind;
            ///
            /// let mut uf = UnionFind::new(5);
            /// assert_eq!(uf.elem_num(), 5);
            /// ```
            pub fn elem_num(&self) -> usize {
                self.parent_or_size.len()
            }
        }
    }

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
