use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut dists = vec![0; 2 * n + 1];
    for (i, &a) in enumerate(&aa) {
        let dist = dists[a] + 1;
        dists[2 * i + 1] = dist;
        dists[2 * i + 2] = dist;
    }

    println!("{}", dists.iter().join("\n"));
}
