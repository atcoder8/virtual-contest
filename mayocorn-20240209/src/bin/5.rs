use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &abc {
        graph[u].push((v, weight));
    }

    let search_min_dist = |start: usize| {
        let mut heap = BinaryHeap::from([(Reverse(0), start)]);
        let mut dists: Vec<Option<usize>> = vec![None; n];
        while let Some((Reverse(cand_dist), cur)) = heap.pop() {
            if dists[cur].is_some() {
                if cur == start {
                    return Some(cand_dist);
                }

                continue;
            }

            dists[cur] = Some(cand_dist);

            heap.extend(
                graph[cur]
                    .iter()
                    .map(|&(next, weight)| (Reverse(cand_dist + weight), next)),
            );
        }

        None
    };

    let ans = (0..n)
        .map(|start| search_min_dist(start))
        .map(|dist| match dist {
            Some(dist) => dist.to_string(),
            None => "-1".to_owned(),
        })
        .join("\n");
    println!("{}", ans);
}
