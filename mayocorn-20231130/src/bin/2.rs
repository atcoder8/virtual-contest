use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut lose = vec![false; n];
    for &(_, b) in &ab {
        lose[b] = true;
    }

    let ans = (0..n).filter(|&i| !lose[i]).collect_vec();
    if ans.len() == 1 {
        println!("{}", ans[0] + 1);
    } else {
        println!("-1");
    }
}
