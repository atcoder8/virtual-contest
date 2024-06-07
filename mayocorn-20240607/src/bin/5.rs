use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut less: Option<usize> = None;
    let mut equal = 0_usize;

    for exp in (0..40).rev() {
        let mut next_less: Option<usize> = None;

        let raised = 2_usize.pow(exp);

        let one_cnt = aa.iter().filter(|&&a| a >> exp & 1 == 1).count();

        // less -> less
        if let Some(less) = less {
            chmax_for_option(&mut next_less, less + one_cnt.max(n - one_cnt) * raised);
        }

        // equal -> less
        if k & raised != 0 {
            chmax_for_option(&mut next_less, equal + one_cnt * raised);
        }

        less = next_less;

        // equal -> equal
        if k & raised == 0 {
            equal += one_cnt * raised;
        } else {
            equal += (n - one_cnt) * raised;
        }
    }

    let ans = less.unwrap_or(0).max(equal);
    println!("{}", ans);
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
