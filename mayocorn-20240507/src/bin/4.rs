use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = (0..=n).collect_vec();
    for base in [6, 9] {
        let mut raised = base;
        while raised <= n {
            for i in raised..=n {
                dp[i] = dp[i].min(dp[i - raised] + 1);
            }

            raised *= base;
        }
    }

    println!("{}", dp[n]);
}
