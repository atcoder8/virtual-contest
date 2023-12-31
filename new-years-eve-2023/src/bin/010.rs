use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n - 1],
    }

    let mut counts = vec![0; n];
    for &a in &aa {
        counts[a] += 1;
    }
    println!("{}", counts.iter().join("\n"));
}
