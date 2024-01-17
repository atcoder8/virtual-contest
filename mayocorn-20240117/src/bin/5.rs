use std::{cmp::Reverse, collections::BinaryHeap};

use im_rc::hashset;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (x, y, z, k): (usize, usize, usize, usize),
        mut aa: [usize; x],
        mut bb: [usize; y],
        mut cc: [usize; z],
    }

    aa.sort_unstable_by_key(|&a| Reverse(a));
    bb.sort_unstable_by_key(|&b| Reverse(b));
    cc.sort_unstable_by_key(|&c| Reverse(c));

    let mut sum_scores = vec![];
    let mut heap = BinaryHeap::from([(aa[0] + bb[0] + cc[0], 0, 0, 0)]);
    let mut used = hashset![(0, 0, 0)];
    for _ in 0..k {
        let (sum_score, i, j, k) = heap.pop().unwrap();
        sum_scores.push(sum_score);

        if i < x - 1 && !used.contains(&(i + 1, j, k)) {
            heap.push((aa[i + 1] + bb[j] + cc[k], i + 1, j, k));
            used.insert((i + 1, j, k));
        }

        if j < y - 1 && !used.contains(&(i, j + 1, k)) {
            heap.push((aa[i] + bb[j + 1] + cc[k], i, j + 1, k));
            used.insert((i, j + 1, k));
        }

        if k < z - 1 && !used.contains(&(i, j, k + 1)) {
            heap.push((aa[i] + bb[j] + cc[k + 1], i, j, k + 1));
            used.insert((i, j, k + 1));
        }
    }

    println!("{}", sum_scores.iter().join("\n"));
}
