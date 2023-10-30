use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (m, n): (usize, usize),
        mut pp: [usize; m],
        ce: [(usize, usize); n],
    }

    pp.sort_unstable_by_key(|&p| Reverse(p));

    // dp[i]: i個のまんじゅうを詰めるのに必要な箱の最小コスト(不可能な場合はNone)
    let mut dp = vec![None; m + 1];
    dp[0] = Some(0);
    for &(c, e) in &ce {
        for i in (0..m).rev() {
            if let Some(cost) = dp[i] {
                let cand_cost = cost + e;
                let next_cost = &mut dp[(i + c).min(m)];
                if next_cost.is_none() || cand_cost < next_cost.unwrap() {
                    *next_cost = Some(cand_cost);
                }
            }
        }
    }
    for i in (0..m).rev() {
        if let Some(cost) = dp[i + 1] {
            let prev_cost = &mut dp[i];
            if prev_cost.is_none() || cost < prev_cost.unwrap() {
                *prev_cost = Some(cost);
            }
        }
    }

    let mut ans = 0;
    let mut sum_p = 0;
    for (i, &p) in pp.iter().enumerate() {
        sum_p += p;
        if let Some(cost) = dp[i + 1] {
            ans = ans.max(sum_p.saturating_sub(cost));
        }
    }

    println!("{}", ans);
}
