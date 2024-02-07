use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
    }

    let mut dp: Vec<Vec<Option<i64>>> = vec![vec![None; w]; h];
    dp[h - 1][w - 1] = Some(0);
    for (row, col) in iproduct!((0..h).rev(), (0..w).rev()) {
        if (row + col) % 2 == 1 {
            let cand = dp[row][col].unwrap() + if aaa[row][col] == '+' { 1 } else { -1 };

            if row > 0 {
                chmax_for_option(&mut dp[row - 1][col], cand);
            }

            if col > 0 {
                chmax_for_option(&mut dp[row][col - 1], cand);
            }
        } else {
            let cand = dp[row][col].unwrap() + if aaa[row][col] == '+' { -1 } else { 1 };

            if row > 0 {
                chmin_for_option(&mut dp[row - 1][col], cand);
            }

            if col > 0 {
                chmin_for_option(&mut dp[row][col - 1], cand);
            }
        }
    }

    let ans = match dp[0][0].unwrap().signum() {
        1 => "Takahashi",
        -1 => "Aoki",
        _ => "Draw",
    };
    println!("{}", ans);
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}

/// If `value` is `None` or contains a value less than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmax_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost >= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
