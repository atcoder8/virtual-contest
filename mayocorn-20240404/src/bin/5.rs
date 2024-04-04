use itertools::iproduct;
use ndarray::Array;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w, k): (usize, usize, usize),
        rcv: [(Usize1, Usize1, usize); k],
    }

    let mut item_mat = Array::from_elem((h, w), 0);
    for &(r, c, v) in &rcv {
        item_mat[(r, c)] = v;
    }

    let mut dp = Array::from_elem((h, w), [0; 4]);
    dp[(0, 0)][1] = item_mat[(0, 0)];

    for (row, col) in iproduct!(0..h, 0..w) {
        for item_num in 0..=3 {
            let cur_score = dp[(row, col)][item_num];

            if row < h - 1 {
                chmax!(dp[(row + 1, col)][0], cur_score);
                chmax!(dp[(row + 1, col)][1], cur_score + item_mat[(row + 1, col)]);
            }

            if col < w - 1 {
                chmax!(dp[(row, col + 1)][item_num], cur_score);

                if item_num < 3 {
                    chmax!(
                        dp[(row, col + 1)][item_num + 1],
                        cur_score + item_mat[(row, col + 1)]
                    );
                }
            }
        }
    }

    println!("{}", dp[(h - 1, w - 1)].iter().max().unwrap());
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
