use std::io::Write;

use itertools::Itertools;
use proconio::{input_interactive, marker::Chars};

fn main() {
    input_interactive!(n: usize);

    let m = calc_ceil_log2(n);

    println!("{}", m);
    std::io::stdout().flush().unwrap();

    let juice_each_friend = (0..m)
        .map(|friend| (0..n).filter(|bit| bit >> friend & 1 == 1).collect_vec())
        .collect_vec();

    for juice in &juice_each_friend {
        println!("{} {}", juice.len(), juice.iter().map(|v| v + 1).join(" "));
    }
    std::io::stdout().flush().unwrap();

    input_interactive!(s: Chars);

    let ans = (0..m)
        .filter(|&i| s[i] == '1')
        .map(|i| 1_usize << i)
        .sum::<usize>()
        + 1;
    println!("{}", ans);
}

pub fn calc_ceil_log2(n: usize) -> usize {
    (0..).find(|&i| n.saturating_sub(1) >> i == 0).unwrap()
}
