use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        ccc: [Chars; 2],
    }

    let ans = iproduct!(0..2, 0..3).all(|(row, col)| ccc[row][col] == ccc[1 - row][2 - col]);
    println!("{}", if ans { "YES" } else { "NO" });
}
