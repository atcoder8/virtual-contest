use std::cmp::Reverse;

use proconio::{input, marker::Usize1};

const MAX_K: usize = 20;

fn main() {
    input! {
        (n, q): (usize, usize),
        xx: [usize; n],
        ab: [(Usize1, Usize1); n - 1],
        vk: [(Usize1, Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut ranking = vec![vec![]; n];

    rec(&graph, &xx, &mut ranking, None, 0);

    for &(v, k) in &vk {
        println!("{}", ranking[v][k]);
    }
}

fn rec(
    graph: &Vec<Vec<usize>>,
    xx: &Vec<usize>,
    ranking: &mut Vec<Vec<usize>>,
    par: Option<usize>,
    cur: usize,
) {
    let mut cur_ranking = vec![xx[cur]];

    for &next in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        rec(graph, xx, ranking, Some(cur), next);
        cur_ranking.append(&mut ranking[next].clone());
    }

    cur_ranking.sort_unstable_by_key(|&x| Reverse(x));

    ranking[cur] = cur_ranking.into_iter().take(MAX_K).collect();
}
