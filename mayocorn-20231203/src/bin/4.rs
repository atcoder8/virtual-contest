use std::collections::BinaryHeap;

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, usize); k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut heap = BinaryHeap::new();
    for &(p, h) in &ph {
        heap.push((h, p));
    }

    let mut guarded = FixedBitSet::with_capacity(n);
    while let Some((rem, cur)) = heap.pop() {
        if guarded[cur] {
            continue;
        }

        guarded.insert(cur);

        if rem != 0 {
            for &next in &graph[cur] {
                heap.push((rem - 1, next));
            }
        }
    }

    println!(
        "{}\n{}",
        guarded.count_ones(..),
        (1..=n).filter(|i| guarded[i - 1]).join(" ")
    );
}
