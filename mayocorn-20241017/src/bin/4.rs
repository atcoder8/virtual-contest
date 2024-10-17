use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

const MAX_K: usize = 20;

fn main() {
    input! {
        (n, q): (usize, usize),
        xx: [usize; n],
        ab: [(Usize1, Usize1); n - 1],
        vk: [(Usize1, usize); q],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut top20_per_node = vec![vec![]; n];
    find_top20(&graph, &xx, None, 0, &mut top20_per_node);

    let ans = vk.iter().map(|&(v, k)| top20_per_node[v][k - 1]).join("\n");
    println!("{}", ans);
}

fn find_top20(
    graph: &[Vec<usize>],
    labels: &[usize],
    parent: Option<usize>,
    current: usize,
    top20_per_node: &mut [Vec<usize>],
) {
    let mut top20 = vec![labels[current]];
    for &next in &graph[current] {
        if Some(next) != parent {
            find_top20(graph, labels, Some(current), next, top20_per_node);
        }
        top20.extend(&top20_per_node[next]);
    }
    top20.sort_unstable_by_key(|&label| Reverse(label));
    top20.truncate(MAX_K);
    top20_per_node[current] = top20;
}
