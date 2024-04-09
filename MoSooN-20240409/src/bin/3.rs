use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        ab: [(usize, usize); n],
    }

    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for &(a, b) in &ab {
        for _ in 0..b {
            for i in (a..=x).rev() {
                dp[i] |= dp[i - a];
            }
        }
    }

    println!("{}", if dp[x] { "Yes" } else { "No" });
}
