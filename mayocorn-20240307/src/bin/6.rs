use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: i64 = 10_i64.pow(15);

fn main() {
    input! {
        n: usize,
        dd: [usize; n],
        mut uvw: [(Usize1, Usize1, i64); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvw {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let (not_connected_score, connected_score) = dfs(&graph, &dd, None, 0);
    println!("{}", not_connected_score.max(connected_score));
}

fn dfs(graph: &[Vec<(usize, i64)>], dd: &[usize], par: Option<usize>, cur: usize) -> (i64, i64) {
    let mut scores = vec![];

    for &(next, weight) in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        let (le_score, lt_score) = dfs(graph, dd, Some(cur), next);

        let not_connected_score = le_score.max(lt_score);
        let connected_score = lt_score + weight;

        scores.push((not_connected_score, connected_score));
    }

    let base_sum_score = scores.iter().map(|v| v.0).sum::<i64>();

    let diff_scores = scores
        .iter()
        .map(|&(not_connected_score, connected_score)| connected_score - not_connected_score)
        .filter(|&diff_score| diff_score > 0)
        .sorted_unstable_by_key(|&diff_score| Reverse(diff_score))
        .collect_vec();

    let le_score = base_sum_score + diff_scores.iter().take(dd[cur]).sum::<i64>();
    let lt_score = if dd[cur] != 0 {
        base_sum_score + diff_scores.iter().take(dd[cur] - 1).sum::<i64>()
    } else {
        -INF
    };

    (le_score, lt_score)
}
