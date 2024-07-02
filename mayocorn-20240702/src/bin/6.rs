use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let calc_dists = |start: usize| {
        let mut dists = vec![None; n + 1];
        let mut queue = VecDeque::from([(start, 0_usize)]);
        while let Some((cur, cand_dist)) = queue.pop_front() {
            if dists[cur].is_some_and(|dist| cand_dist >= dist) {
                continue;
            }

            dists[cur] = Some(cand_dist);

            if cur != 0 {
                queue.extend(graph[cur].iter().map(|&next| (next, cand_dist + 1)));
            }
        }

        dists
    };

    let dists_from_start = calc_dists(1);
    let dists_from_goal = calc_dists(n);

    let calc_min_dist = |i: usize| {
        let mut candidates = vec![];

        // 1 -> N
        if let Some(dist) = dists_from_start[n] {
            candidates.push(dist);
        }

        // 1 -> i -> j -> N
        if let (Some(dist1), Some(dist2)) = (dists_from_start[i], dists_from_goal[0]) {
            candidates.push(dist1 + dist2);
        }

        // 1 -> j -> i -> N
        if let (Some(dist1), Some(dist2)) = (dists_from_start[0], dists_from_goal[i]) {
            candidates.push(dist1 + dist2);
        }

        // 1 -> i -> N
        if let (Some(dist1), Some(dist2)) = (dists_from_start[0], dists_from_goal[0]) {
            candidates.push(dist1 + dist2);
        }

        candidates.into_iter().min()
    };

    let ans = (1..=n)
        .map(|i| match calc_min_dist(i) {
            Some(min_dist) => min_dist.to_string(),
            None => "-1".to_string(),
        })
        .join(" ");
    println!("{}", ans);
}
