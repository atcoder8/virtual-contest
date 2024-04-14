use itertools::enumerate;
use ndarray::Array3;
use proconio::input;

// k個の素材の合計がsである場合、ある時刻tにs+tk=Xとなる必要がある

fn main() {
    input! {
        (n, x): (usize, usize),
        aa: [usize; n],
    }

    // dp[i][j][k]: i個の素材の魔力の合計をj+1で割った余りがk
    let mut dp = Array3::<Option<usize>>::default((n + 1, n, n));
    for (i, &a) in enumerate(&aa) {
        for cnt in (1..=i).rev() {
            for modulus in 1..=n {
                for rem in 0..modulus {
                    if let Some(sum) = dp[(cnt, modulus - 1, rem)] {
                        chmax_for_option(
                            &mut dp[(cnt + 1, modulus - 1, (rem + a) % modulus)],
                            sum + a,
                        );
                    }
                }
            }
        }

        for modulus in 1..=n {
            chmax_for_option(&mut dp[(1, modulus - 1, a % modulus)], a);
        }
    }

    let ans = (1..=n)
        .filter_map(|cnt| {
            let Some(sum) = dp[(cnt, cnt - 1, x % cnt)] else {
            return None;
        };

            assert_eq!((x - sum) % cnt, 0);

            Some((x - sum) / cnt)
        })
        .min()
        .unwrap();
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
