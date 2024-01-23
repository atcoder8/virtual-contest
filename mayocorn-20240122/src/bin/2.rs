use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pp: [usize; n],
    }

    let rev_pos = (0..n - 1).rev().find(|&i| pp[i] > pp[i + 1]).unwrap();
    let swap_pos = (rev_pos + 1..n)
        .filter(|&i| pp[i] < pp[rev_pos])
        .max_by_key(|&i| pp[i])
        .unwrap();
    pp.swap(rev_pos, swap_pos);
    pp[rev_pos + 1..].sort_unstable_by_key(|&p| Reverse(p));

    println!("{}", pp.iter().join(" "));
}
