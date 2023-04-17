use proconio::input;

use crate::atcoder8_library::lis::StronglyLIS;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut lis = StronglyLIS::new();

    let mut acc_lis = vec![];
    for &a in &aa {
        lis.push(a);
        acc_lis.push(lis.lis_len());
    }

    let mut rev_lis = StronglyLIS::new();
    let mut rev_acc_lis = vec![];
    for &a in aa.iter().rev() {
        rev_lis.push(a);
        rev_acc_lis.push(rev_lis.lis_len());
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(acc_lis[i] + rev_acc_lis[n - 1 - i] - 1);
    }

    println!("{}", ans);
}

pub mod atcoder8_library {
    pub mod lis {
        use superslice::Ext;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            dp: Vec<T>,
        }

        impl<T> WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            pub fn new() -> Self {
                Self { dp: vec![] }
            }

            pub fn push(&mut self, x: T) {
                let idx = self.dp.upper_bound(&x);
                if idx < self.dp.len() {
                    self.dp[idx] = x;
                } else {
                    self.dp.push(x);
                }
            }

            pub fn lis_len(&self) -> usize {
                self.dp.len()
            }
        }

        impl<T> Default for WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            fn default() -> Self {
                WeeklyLIS::new()
            }
        }

        impl<T> From<Vec<T>> for WeeklyLIS<T>
        where
            T: Clone + Ord,
        {
            fn from(seq: Vec<T>) -> Self {
                let mut lis = Self::default();
                for x in seq {
                    lis.push(x);
                }
                lis
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            dp: Vec<T>,
        }

        impl<T> StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            pub fn new() -> Self {
                Self { dp: vec![] }
            }

            pub fn push(&mut self, x: T) {
                let idx = self.dp.lower_bound(&x);
                if idx < self.dp.len() {
                    self.dp[idx] = x;
                } else {
                    self.dp.push(x);
                }
            }

            pub fn lis_len(&self) -> usize {
                self.dp.len()
            }
        }

        impl<T> Default for StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            fn default() -> Self {
                StronglyLIS::new()
            }
        }

        impl<T> From<Vec<T>> for StronglyLIS<T>
        where
            T: Clone + Ord,
        {
            fn from(seq: Vec<T>) -> Self {
                let mut lis = Self::default();
                for x in seq {
                    lis.push(x);
                }
                lis
            }
        }
    }
}
