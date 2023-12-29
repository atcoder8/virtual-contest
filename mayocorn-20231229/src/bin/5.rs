use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        acc: [(usize, [Usize1]); m],
    }

    let mut dp = vec![None; 1 << n];
    dp[0] = Some(0);
    for (a, cc) in acc {
        for from in (0..1 << n).rev() {
            let Some(sum_cost) = dp[from] else { continue; };

            let add_bit = cc.iter().map(|&c| 1 << c).sum::<usize>();
            update_cost(&mut dp[from | add_bit], sum_cost + a);
        }
    }

    match dp.last().unwrap() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
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
