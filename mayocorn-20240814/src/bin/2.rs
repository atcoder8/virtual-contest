use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        mut ss: [Chars; h],
        mut tt: [Chars; h],
    }

    let mut t_ss = vec![vec!['\0'; h]; w];
    let mut t_tt = vec![vec!['\0'; h]; w];
    for (row, col) in iproduct!(0..h, 0..w) {
        t_ss[col][row] = ss[row][col];
        t_tt[col][row] = tt[row][col];
    }

    t_ss.sort_unstable();
    t_tt.sort_unstable();

    println!("{}", if t_ss == t_tt { "Yes" } else { "No" });
}
