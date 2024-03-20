use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
        qq: [Usize1; n],
    }

    let idx1 = (0..n).permutations(n).position(|perm| perm == pp).unwrap();
    let idx2 = (0..n).permutations(n).position(|perm| perm == qq).unwrap();

    let ans = idx1.abs_diff(idx2);
    println!("{}", ans);
}
