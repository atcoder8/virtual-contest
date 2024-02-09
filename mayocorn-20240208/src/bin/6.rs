use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(usize, usize); m],
    }

    let mut teleporters = vec![];
    let mut edges = vec![];
    for &(u, v) in &uv {
        if u == 0 {
            teleporters.push(v - 1);
        } else {
            edges.push((u - 1, v - 1));
        }
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let search_min_dists = |start: usize| {
        let mut dists: Vec<Option<usize>> = vec![None; n];
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((cur, cand_dist)) = queue.pop_front() {
            if dists[cur].is_some() {
                continue;
            }

            dists[cur] = Some(cand_dist);

            queue.extend(graph[cur].iter().map(|&next| (next, cand_dist + 1)));
        }

        dists
    };

    let dists_from_start = search_min_dists(0);
    let dists_to_goal = search_min_dists(n - 1);

    let min_dist_from_start = teleporters
        .iter()
        .filter_map(|&teleporter| dists_from_start[teleporter])
        .min();
    let min_dist_to_goal = teleporters
        .iter()
        .filter_map(|&teleporter| dists_to_goal[teleporter])
        .min();

    let mut ans = vec![dists_from_start[n - 1]; n];
    for i in 0..n {
        if let (Some(dist_from_start), Some(min_dist_to_goal)) =
            (dists_from_start[i], min_dist_to_goal)
        {
            let cand_dist = dist_from_start + 1 + min_dist_to_goal;
            chmin_for_option(&mut ans[i], cand_dist);
        }

        if let (Some(min_dist_from_start), Some(dist_to_goal)) =
            (min_dist_from_start, dists_to_goal[i])
        {
            let cand_dist = min_dist_from_start + 1 + dist_to_goal;
            chmin_for_option(&mut ans[i], cand_dist);
        }

        if let (Some(min_dist_from_start), Some(min_dist_from_goal)) =
            (min_dist_from_start, min_dist_to_goal)
        {
            let cand_dist = min_dist_from_start + 2 + min_dist_from_goal;
            chmin_for_option(&mut ans[i], cand_dist);
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|&dist| match dist {
                Some(dist) => format!("{dist}"),
                None => "-1".to_owned(),
            })
            .join(" ")
    );
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
