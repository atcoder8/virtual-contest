use itertools::{chain, Itertools};
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        aaa: [Chars; n],
    }

    let perimeter = chain!(
        (0..n).map(|col| (0, col)),
        (0..n).map(|row| (row, n - 1)),
        (0..n).rev().map(|col| (n - 1, col)),
        (0..n).rev().map(|row| (row, 0))
    )
    .dedup()
    .collect_vec();
    let mut bbb = Array2::from_shape_fn((n, n), |(row, col)| aaa[row][col]);
    for (&coord1, &coord2) in perimeter.iter().tuple_windows() {
        bbb[coord2] = aaa[coord1.0][coord1.1];
    }

    let ans = bbb
        .axis_iter(Axis(0))
        .map(|bb| bb.iter().collect::<String>())
        .join("\n");
    println!("{}", ans);
}
