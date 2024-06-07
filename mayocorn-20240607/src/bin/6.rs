use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
        x: String,
    }

    let mut dp = [false; 7];
    dp[0] = true;

    let mut raised = 1;
    for (d, c) in s.chars().rev().zip(x.chars().rev()) {
        let mut next_dp = [false; 7];

        let d = d.to_digit(10).unwrap() as usize;

        if c == 'A' {
            for from in 0..7 {
                next_dp[from] = dp[from] && dp[(from + d * raised) % 7];
            }
        } else {
            for from in 0..7 {
                next_dp[from] = dp[from] || dp[(from + d * raised) % 7];
            }
        }

        dp = next_dp;

        raised = 10 * raised % 7;
    }

    println!("{}", if dp[0] { "Takahashi" } else { "Aoki" });
}
