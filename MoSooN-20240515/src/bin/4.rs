use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(usize, usize, usize); n],
    }

    let total_z = xyz.iter().map(|v| v.2).sum::<usize>();
    let win_z = xyz
        .iter()
        .map(|&(x, y, z)| z * (x > y) as usize)
        .sum::<usize>();

    let mut dp: Vec<Option<usize>> = vec![None; total_z + 1];
    dp[win_z] = Some(0);

    for &(x, y, z) in &xyz {
        if x > y {
            continue;
        }

        for sum_z in (z..=total_z).rev() {
            if let Some(prev) = dp[sum_z - z] {
                chmin_for_option(&mut dp[sum_z], prev + (y - x + 1) / 2);
            }
        }
    }

    let ans = dp[(total_z + 1) / 2..]
        .iter()
        .filter_map(|&cost| cost)
        .min()
        .unwrap();
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
