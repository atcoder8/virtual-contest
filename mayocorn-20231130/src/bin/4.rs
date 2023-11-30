use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, k, d): (usize, usize, usize),
        aa: [usize; n],
    }

    // dp[i][j]: i個使ってdで割った倍数がjになるものの最大値
    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; d]; k + 1];
    dp[0][0] = Some(0);
    for (a, term_num, rem) in iproduct!(aa, (0..k).rev(), 0..d) {
        if let Some(score) = dp[term_num][rem] {
            update_score(&mut dp[term_num + 1][(rem + a) % d], score + a);
        }
    }

    match dp[k][0] {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn update_score(score: &mut Option<usize>, cand_score: usize) {
    if score.is_none() || cand_score > score.unwrap() {
        *score = Some(cand_score);
    }
}
