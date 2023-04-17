use proconio::input;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        aa: [usize; n],
    }

    let mut dp: Vec<Option<(usize, usize, usize)>> = vec![None; m];
    dp[0] = Some((aa[0], aa[0], 0));
    for &a in aa[1..].iter().chain(&vec![0]) {
        let mut next_dp: Vec<Option<(usize, usize, usize)>> = vec![None; m];

        for elem_num in 1..m {
            if let Some((min, max, cost)) = dp[elem_num - 1] {
                next_dp[elem_num] = Some((min.min(a), max.max(a), cost));
            }
        }

        for elem_num in 1..=m {
            if let Some((min, max, cost)) = dp[elem_num - 1] {
                let candidate_cost = cost + k + elem_num * (max - min);

                if next_dp[0].is_none() || candidate_cost < next_dp[0].unwrap().2 {
                    next_dp[0] = Some((a, a, candidate_cost));
                }
            }
        }

        dp = next_dp;
    }

    let ans = dp[0].unwrap().2;
    println!("{}", ans);
}
