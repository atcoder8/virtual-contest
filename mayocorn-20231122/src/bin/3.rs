use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        (h1, w1): (usize, usize),
        aaa: [[usize; w1]; h1],
        (h2, w2): (usize, usize),
        bbb: [[usize; w2]; h2],
    }

    let diff_h = h1 - h2;
    let diff_w = w1 - w2;

    let is_ok = |del_rows: &[usize], del_cols: &[usize]| {
        let mut shift_row = 0;
        for i in 0..h1 {
            if shift_row < diff_h && i == del_rows[shift_row] {
                shift_row += 1;
                continue;
            }

            let mut shift_col = 0;
            for j in 0..w1 {
                if shift_col < diff_w && j == del_cols[shift_col] {
                    shift_col += 1;
                    continue;
                }

                if aaa[i][j] != bbb[i - shift_row][j - shift_col] {
                    return false;
                }
            }
        }

        true
    };

    let ans = iproduct!((0..h1).combinations(diff_h), (0..w1).combinations(diff_w))
        .any(|(del_rows, del_cols)| is_ok(&del_rows, &del_cols));
    println!("{}", if ans { "Yes" } else { "No" });
}
