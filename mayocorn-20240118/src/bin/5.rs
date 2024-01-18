use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        match solve() {
            Some(ans) => println!("{}", ans),
            None => println!("-1"),
        }
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        cc: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    if cc[0] == cc[n - 1] {
        return None;
    }

    let mut costs = vec![vec![None; n]; n];
    let mut queue = VecDeque::from([(0, n - 1, 0)]);
    while let Some((node1, node2, cost)) = queue.pop_front() {
        if costs[node1][node2].is_some() {
            continue;
        }

        costs[node1][node2] = Some(cost);

        for (&next_node_1, &next_node_2) in iproduct!(&graph[node1], &graph[node2]) {
            if cc[next_node_1] != cc[next_node_2] {
                queue.push_back((next_node_1, next_node_2, cost + 1));
            }
        }
    }

    costs[n - 1][0]
}
