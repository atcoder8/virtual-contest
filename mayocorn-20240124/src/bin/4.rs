use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k, p): (usize, u32, usize),
        caa: [(usize, [usize; k]); n],
    }

    let mut dp = vec![None; (p + 1).pow(k)];
    dp[0] = Some(0);
    for (c, aa) in caa {
        for params in (0..(p + 1).pow(k)).rev() {
            let Some(sum_cost) = dp[params] else { continue; };

            let mut next_params = 0;
            for (param_idx, &a) in enumerate(&aa) {
                let param_idx = param_idx as u32;
                let param = params % (p + 1).pow(param_idx + 1) / (p + 1).pow(param_idx);
                let next_param = (param + a).min(p);
                next_params += next_param * (p + 1).pow(param_idx);
            }

            update_cost(&mut dp[next_params], sum_cost + c);
        }
    }

    match dp[(p + 1).pow(k) - 1] {
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
