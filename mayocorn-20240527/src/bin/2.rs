use itertools::{chain, iproduct, Itertools};
use ndarray::{s, Array2, ArrayView2};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let grid = Array2::from_shape_fn((n, m), |(row, col)| ss[row][col] == '#');

    let is_ok = |grid: ArrayView2<bool>| {
        if chain!(grid.slice(s![0..3, 0..3]), grid.slice(s![6..9, 6..9])).any(|black| !black) {
            return false;
        }

        for i in 0..=3 {
            if grid[(i, 3)] || grid[(3, i)] || grid[(8 - i, 5)] || grid[(5, 8 - i)] {
                return false;
            }
        }

        true
    };

    let ans = iproduct!(0..=n - 9, 0..=m - 9)
        .filter(|&(top, left)| is_ok(grid.slice(s![top..top + 9, left..left + 9])))
        .map(|(top, left)| format!("{} {}", top + 1, left + 1))
        .join("\n");
    println!("{}", ans);
}
