use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k):(usize, usize, usize),
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, usize); k],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut nodes = vec![0; n];
    let mut heap: BinaryHeap<(usize, usize)> = ph.iter().map(|&(p, h)| (h + 1, p)).collect();

    while let Some((val, cur)) = heap.pop() {
        if nodes[cur] >= val {
            continue;
        }

        nodes[cur] = val;

        if val > 1 {
            heap.extend(graph[cur].iter().map(|&next| (val - 1, next)));
        }
    }

    let ans = nodes
        .iter()
        .positions(|&val| val != 0)
        .map(|i| i + 1)
        .collect_vec();
    println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
