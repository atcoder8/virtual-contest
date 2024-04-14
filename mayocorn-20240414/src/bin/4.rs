use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

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

    let mut top_each_node = vec![vec![]; n];
    dfs(&graph, &xx, None, 0, &mut top_each_node);

    let ans = vk.iter().map(|&(v, k)| top_each_node[v][k - 1]).join("\n");
    println!("{}", ans);
}

fn dfs(
    graph: &[Vec<usize>],
    xx: &[usize],
    par: Option<usize>,
    cur: usize,
    top_each_node: &mut [Vec<usize>],
) {
    let mut top = vec![xx[cur]];
    for &next in &graph[cur] {
        if Some(next) != par {
            dfs(graph, xx, Some(cur), next, top_each_node);
            top.extend(top_each_node[next].clone());
        }
    }
    top.sort_unstable_by_key(|&val| Reverse(val));
    top.truncate(20);

    top_each_node[cur] = top;
}
