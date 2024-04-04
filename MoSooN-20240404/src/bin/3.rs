use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let ans = aa
        .iter()
        .sorted_unstable()
        .dedup_with_count()
        .map(|v| v.0)
        .sorted_unstable_by_key(|&cnt| Reverse(cnt))
        .skip(k)
        .sum::<usize>();
    println!("{}", ans);
}
