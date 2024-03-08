use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        xy: [(i64, i64); n],
    }

    let xx = xy.iter().map(|v| v.0).sorted_unstable().collect_vec();

    let find_min_len = |left: i64, right: i64| {
        let yy = xy
            .iter()
            .filter(|v| left <= v.0 && v.0 <= right)
            .map(|v| v.1)
            .sorted_unstable()
            .collect_vec();

        if yy.len() < k {
            return None;
        }

        let min_len = (0..=yy.len() - k)
            .map(|i| yy[i + k - 1] - yy[i])
            .min()
            .unwrap();
        Some(min_len)
    };

    let ans = xx
        .iter()
        .tuple_combinations()
        .filter_map(|(&left, &right)| find_min_len(left, right).map(|len| (right - left) * len))
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
