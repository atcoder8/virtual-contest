use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut dists = vec![vec![None; n]; n];
    for i in 0..n {
        dists[i][i] = Some(0);
    }
    for &(a, b, c) in &abc {
        dists[a][b] = Some(c);
    }

    let mut ans = 0;
    for mid in 0..n {
        for (from, to) in iproduct!(0..n, 0..n) {
            let (Some(dist1), Some(dist2)) = (dists[from][mid], dists[mid][to]) else {
                continue;
            };

            update_cost(&mut dists[from][to], dist1 + dist2);
        }

        for (from, to) in iproduct!(0..n, 0..n) {
            if let Some(dist) = dists[from][to] {
                ans += dist;
            }
        }
    }

    println!("{}", ans);
}

/// Updates the minimum cost.
/// If `cost` is `None`, always updated to the candidate cost.
///
/// # Arguments
///
/// * `cost` - Reference variable for the cost to be updated.
/// * `cand_cost` - Candidate cost to update.
pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: PartialOrd,
{
    if cost.as_ref().is_some_and(|cost| cost <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
