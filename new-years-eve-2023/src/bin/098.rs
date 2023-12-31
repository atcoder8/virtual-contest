use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        sss: [Chars; n],
    }

    for j in 0..n {
        println!("{}", (0..n).rev().map(|i| sss[i][j]).join(""));
    }
}
