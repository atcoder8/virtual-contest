use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n],
    }

    let ans = (1..=n)
        .sorted_unstable_by_key(|&i| (&sp[i - 1].0, Reverse(sp[i - 1].1)))
        .join("\n");
    println!("{}", ans);
}
