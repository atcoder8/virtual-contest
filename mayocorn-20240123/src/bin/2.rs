use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        bbb: [[Usize1; m]; n],
    }

    let base = bbb[0][0];

    if base % 7 + m > 7 {
        return false;
    }

    for (row, col) in iproduct!(0..n, 0..m) {
        if row > 0 && bbb[row - 1][col] + 7 != bbb[row][col] {
            return false;
        }

        if col > 0 && bbb[row][col - 1] + 1 != bbb[row][col] {
            return false;
        }
    }

    true
}
