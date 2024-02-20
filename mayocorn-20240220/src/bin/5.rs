use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k, q): (usize, usize, usize),
        xy: [(Usize1, usize); q],
    }

    let mut aa = vec![0; n];

    let mut ans = vec![];
    let mut cur = 0;
    let mut large: BTreeSet<(usize, usize)> = (0..k).map(|i| (0, i)).collect();
    let mut small: BTreeSet<(usize, usize)> = (k..n).map(|i| (0, i)).collect();
    for &(x, y) in &xy {
        let prev = (aa[x], x);
        aa[x] = y;

        if large.contains(&prev) {
            cur -= prev.0;
            large.remove(&prev);
            small.insert((y, x));
            let max_small = small.pop_last().unwrap();
            large.insert(max_small);
            cur += max_small.0;
        } else {
            small.remove(&prev);
            small.insert((y, x));

            if let Some(min_large) = large.pop_first() {
                cur -= min_large.0;
                small.insert(min_large);
                let max_small = small.pop_last().unwrap();
                large.insert(max_small);
                cur += max_small.0;
            }
        }

        ans.push(cur);
    }

    println!("{}", ans.iter().join("\n"));
}
