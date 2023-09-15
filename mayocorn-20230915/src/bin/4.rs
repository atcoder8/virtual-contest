use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, q): (usize, usize, usize),
        mut wv: [(usize, usize); n],
        xx: [usize; m],
        lr: [(Usize1, usize); q],
    }

    wv.sort_unstable_by_key(|&(_, v)| Reverse(v));

    for &(l, r) in &lr {
        let mut ans = 0;
        let mut boxes = xx[..l]
            .iter()
            .chain(xx[r..].iter())
            .cloned()
            .sorted()
            .collect_vec();

        for &(w, v) in &wv {
            if let Some(pos) = boxes.iter().position(|&size| size >= w) {
                ans += v;
                boxes.remove(pos);
            }
        }

        println!("{}", ans);
    }
}
