use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        ccc: [[i64; 3]; 3],
    }

    let hor_sums = (0..3)
        .map(|row| (0..3).map(|col| ccc[row][col]).sum::<i64>())
        .collect_vec();
    let ver_sums = (0..3)
        .map(|col| (0..3).map(|row| ccc[row][col]).sum::<i64>())
        .collect_vec();
    let sum_all = hor_sums.iter().sum::<i64>();

    let sum_ab = sum_all / 3;
    let b = hor_sums[0];
    let a = sum_ab - b;

    let aa = hor_sums
        .iter()
        .map(|sum_row| (sum_row - b) / 3)
        .collect_vec();
    let bb = ver_sums
        .iter()
        .map(|sum_col| (sum_col - a) / 3)
        .collect_vec();

    iproduct!(0..3, 0..3).all(|(i, j)| ccc[i][j] == aa[i] + bb[j])
}
