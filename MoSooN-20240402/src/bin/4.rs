use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m, l): (usize, usize, usize),
        aa: [usize; n],
        bb: [usize; m],
        cd: [(Usize1, Usize1); l],
    }

    let bad: BTreeSet<(usize, usize)> = cd.into_iter().collect();

    let jb = enumerate(bb)
        .sorted_unstable_by_key(|v| Reverse(v.1))
        .collect_vec();

    let mut heap: BinaryHeap<(usize, usize, usize)> =
        enumerate(&aa).map(|(i, &a)| (a + jb[0].1, i, 0)).collect();

    loop {
        let (score, i, b_pos) = heap.pop().unwrap();

        if !bad.contains(&(i, jb[b_pos].0)) {
            return score;
        }

        if b_pos < m - 1 {
            heap.push((aa[i] + jb[b_pos + 1].1, i, b_pos + 1));
        }
    }
}
