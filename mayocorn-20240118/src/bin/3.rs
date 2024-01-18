use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let ans = rec(&graph, None, 0);
    println!("{}", ans);
}

fn rec(graph: &[Vec<usize>], par: Option<usize>, cur: usize) -> usize {
    let counts = graph[cur]
        .iter()
        .filter(|&&next| Some(next) != par)
        .map(|&next| rec(graph, Some(cur), next))
        .collect_vec();
    let sum = counts.iter().sum::<usize>();

    if cur == 0 {
        sum - counts.iter().max().unwrap() + 1
    } else {
        sum + 1
    }
}
