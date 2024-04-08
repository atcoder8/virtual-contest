use itertools::enumerate;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> i64 {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let final_skip = n % 2 + 1;

    let mut dp = vec![vec![-10_i64.pow(16); final_skip + 1]; n];

    for skip in 0..=final_skip {
        dp[skip][skip] = aa[skip];
    }

    for (i, &a) in enumerate(&aa).skip(final_skip) {
        for prev_skip in 0..=final_skip {
            for skip in prev_skip..=final_skip {
                let add_skip = skip - prev_skip;
                if i >= add_skip + 2 {
                    chmax!(dp[i][skip], dp[i - 2 - add_skip][prev_skip] + a);
                }
            }
        }
    }

    (0..=final_skip)
        .map(|skip| dp[n - 1 - skip][final_skip - skip])
        .max()
        .unwrap()
}

/// If the right-hand side is greater than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is less than or equal to the left-hand side,
/// does nothing and returns `false`.
///
/// The left-hand and right-hand sides must be the same type
/// and must implement the `Clone` trait.
///
/// # Examples
///
/// ```
/// let mut x = 5;
///
/// assert_eq!(chmax!(x, 3), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmax!(x, 7), true);
/// assert_eq!(x, 7);
/// ```
#[macro_export]
macro_rules! chmax {
    ($lhs: expr, $rhs: expr) => {
        if $rhs > $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
