use itertools::iproduct;
use ndarray::Array;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &abc {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let mut dist_mat = Array::from_elem((n, n), 10_usize.pow(7));
    for i in 0..n {
        dist_mat[(i, i)] = 0;
    }
    for &(u, v, weight) in &abc {
        dist_mat[(u, v)] = weight;
        dist_mat[(v, u)] = weight;
    }
    for (mid, from, to) in iproduct!(0..n, 0..n, 0..n) {
        chmin!(
            dist_mat[(from, to)],
            dist_mat[(from, mid)] + dist_mat[(mid, to)]
        );
    }

    let ans = abc
        .iter()
        .filter(|&&(a, b, c)| c > dist_mat[(a, b)])
        .count();
    println!("{}", ans);
}

/// If the right-hand side is less than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is greater than or equal to the left-hand side,
/// does nothing and returns `false`.
///
/// The left-hand and right-hand sides must be the same type
/// and must implement the `Clone` trait.
///
/// # Examples
///
/// ```
/// let mut x = 5;
///
/// assert_eq!(chmin!(x, 7), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmin!(x, 3), true);
/// assert_eq!(x, 3);
/// ```
#[macro_export]
macro_rules! chmin {
    ($lhs: expr, $rhs: expr) => {
        if $rhs < $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
