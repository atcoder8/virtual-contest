use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m, k): (usize, usize, usize),
        uva: [(Usize1, Usize1, usize); m],
        ss: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, a) in &uva {
        graph[u].push((v, a));
        graph[v].push((u, a));
    }

    let mut switches = vec![false; n];
    for &s in &ss {
        switches[s] = true;
    }

    let mut dp: Vec<[Option<usize>; 2]> = vec![[None; 2]; n];

    let mut queue = VecDeque::from([(0, false, 0)]);
    while let Some((cur, switch, cost)) = queue.pop_front() {
        let cur_cost = &mut dp[cur][switch as usize];

        if cur_cost.is_some() {
            continue;
        }

        *cur_cost = Some(cost);

        if switches[cur] {
            dp[cur][!switch as usize] = Some(cost);
        }

        queue.extend(
            graph[cur]
                .iter()
                .filter(|v| v.1 != switch as usize)
                .map(|v| (v.0, switch, cost + 1)),
        );

        if switches[cur] {
            queue.extend(
                graph[cur]
                    .iter()
                    .filter(|v| v.1 == switch as usize)
                    .map(|v| (v.0, !switch, cost + 1)),
            );
        }
    }

    dp[n - 1].iter().filter_map(|&v| v).min()
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
