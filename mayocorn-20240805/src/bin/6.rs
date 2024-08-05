use proconio::{input, marker::Usize1};

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<i64> {
    input! {
        (n, m, p): (usize, usize, i64),
        abc: [(Usize1, Usize1, i64); m],
    }

    let mut inv_graph = vec![vec![]; n];
    for &(a, b, _) in &abc {
        inv_graph[b].push(a);
    }

    let mut visited = vec![false; n];
    let mut stack = vec![n - 1];
    while let Some(cur) = stack.pop() {
        if visited[cur] {
            continue;
        }

        visited[cur] = true;

        stack.extend(inv_graph[cur].iter().cloned());
    }

    let mut costs: Vec<Option<i64>> = vec![None; n];
    costs[0] = Some(0);

    for cnt in 0..n + 1 {
        let mut updated = false;
        let mut updated_costs = costs.clone();

        for &(a, b, c) in &abc {
            if !visited[b] {
                continue;
            }

            let Some(cost) = costs[a] else {
                continue;
            };

            updated |= chmin_for_option(&mut updated_costs[b], cost + p - c);
        }

        if !updated {
            break;
        }

        if cnt == n {
            return None;
        }

        costs = updated_costs;
    }

    Some((-costs[n - 1].unwrap()).max(0))
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
