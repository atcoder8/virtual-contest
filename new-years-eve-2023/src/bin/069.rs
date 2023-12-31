use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        ss: [Chars; 3],
    }

    let ans = ss
        .iter()
        .tuple_windows()
        .all(|(s1, s2)| s1.last() == s2.first());
    println!("{}", if ans { "YES" } else { "NO" });
}
