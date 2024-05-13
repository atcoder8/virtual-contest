use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};
use tap::TapOptional;

fn main() {
    match solve() {
        Some(ans) => println!("{}\n{}", ans.len(), ans.iter().map(|x| x + 1).join("\n")),
        None => println!("-1"),
    }
}

fn solve() -> Option<Vec<usize>> {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    let mut rev_graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        rev_graph[b].push(a);
    }

    let find_cycle = |start: usize| {
        let mut queue = VecDeque::from([(start, 0)]);
        let mut dists: Vec<Option<usize>> = vec![None; n];
        let mut cycle_len = 0;

        while let Some((cur, cand_dist)) = queue.pop_front() {
            if cur == start && cand_dist != 0 {
                cycle_len = cand_dist;
                break;
            }

            let dist = &mut dists[cur];

            if dist.is_none() || cand_dist < dist.unwrap() {
                *dist = Some(cand_dist);
                queue.extend(graph[cur].iter().map(|&adj| (adj, cand_dist + 1)));
            }
        }

        if cycle_len == 0 {
            return None;
        }

        let mut cycle = vec![];
        let mut cur = start;
        let mut dist = cycle_len;

        while dist != 0 {
            let adj = *rev_graph[cur]
                .iter()
                .find(|&&adj| dists[adj] == Some(dist - 1))
                .unwrap();
            cycle.push(adj);
            cur = adj;
            dist -= 1;
        }

        Some(cycle)
    };

    (0..n)
        .filter_map(find_cycle)
        .min_by_key(|cycle| cycle.len())
        .tap_some_mut(|cycle| cycle.sort_unstable())
}
