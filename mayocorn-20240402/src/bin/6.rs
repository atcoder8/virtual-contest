use fixedbitset::FixedBitSet;
use itertools::{iproduct, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        aaa: [Chars; n],
    }

    let mut mat = vec![FixedBitSet::with_capacity(n); n];
    for (row, col) in iproduct!(0..n, 0..n) {
        mat[row].set(col, aaa[row][col] == '1');
    }

    let mut ans = 0;
    for (i, j) in (0..n).tuple_combinations() {
        if aaa[i][j] == '0' {
            continue;
        }

        let mut intersection = mat[i].clone();
        intersection.intersect_with(&mat[j]);

        ans += intersection.count_ones(j + 1..);
    }

    println!("{}", ans);
}
