use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        aa: [usize; n],
    }

    let mut ans = None;
    for k in 1..=n {
        let mut dp = vec![vec![None; k]; k + 1];
        dp[0][0] = Some(0);

        for &a in &aa {
            for (i, j) in iproduct!((0..k).rev(), 0..k) {
                if let Some(sum) = dp[i][j] {
                    update_score(&mut dp[i + 1][(j + a) % k], sum + a);
                }
            }
        }

        if let Some(sum) = dp[k][x % k] {
            update_cost(&mut ans, (x - sum) / k);
        }
    }

    println!("{}", ans.unwrap());
}

fn update_cost(cost: &mut Option<usize>, cand_cost: usize) {
    if cost.is_none() || cand_cost < cost.unwrap() {
        *cost = Some(cand_cost);
    }
}

fn update_score(score: &mut Option<usize>, cand_score: usize) {
    if score.is_none() || cand_score > score.unwrap() {
        *score = Some(cand_score);
    }
}
