use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ans = aa
        .iter()
        .sorted_unstable_by_key(|&&a| Reverse(a))
        .step_by(2)
        .sum::<usize>();
    println!("{}", ans);
}
