use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        rr: [usize; h],
        cc: [usize; w],
        aaa: [Chars; h],
    }

    // dp[(i, j)][k]: マス(i, j)にいて反転の状態がk
    let mut dp = Array2::from_elem((h, w), [10_usize.pow(15); 4]);
    dp[(0, 0)] = [0, rr[0], cc[0], rr[0] + cc[0]];
    for (row, col) in iproduct!(0..h, 0..w) {
        if row > 0 {
            for prev_flip in 0..4 {
                let flip = if aaa[row - 1][col] == aaa[row][col] {
                    prev_flip
                } else {
                    prev_flip ^ 1
                };
                let add_cost = rr[row] * (prev_flip & 1);

                chmin!(
                    dp[(row, col)][prev_flip],
                    dp[(row - 1, col)][flip] + add_cost
                );
            }
        }

        if col > 0 {
            for prev_flip in 0..4 {
                let flip = if aaa[row][col - 1] == aaa[row][col] {
                    prev_flip
                } else {
                    prev_flip ^ 2
                };
                let add_cost = cc[col] * (prev_flip >> 1 & 1);

                chmin!(
                    dp[(row, col)][prev_flip],
                    dp[(row, col - 1)][flip] + add_cost
                );
            }
        }
    }

    let ans = dp[(h - 1, w - 1)].iter().min().unwrap();
    println!("{}", ans);
}

/// If the right-hand side is less than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is greater than or equal to the left-hand side,
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
/// assert_eq!(chmin!(x, 7), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmin!(x, 3), true);
/// assert_eq!(x, 3);
/// ```
#[macro_export]
macro_rules! chmin {
    ($lhs: expr, $rhs: expr) => {
        if $rhs < $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
