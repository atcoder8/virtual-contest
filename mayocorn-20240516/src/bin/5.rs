use itertools::{iproduct, Itertools};
use ndarray::Array2;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, q): (usize, usize),
        grid: [Chars; n],
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut imos = Array2::from_shape_fn((n + 1, n + 1), |(row, col)| {
        (row > 0 && col > 0 && grid[row - 1][col - 1] == 'B') as usize
    });
    for (row, col) in iproduct!(0..=n, 0..n) {
        imos[(row, col + 1)] += imos[(row, col)];
    }
    for (row, col) in iproduct!(0..n, 0..=n) {
        imos[(row + 1, col)] += imos[(row, col)];
    }

    let calc_sum = |height: usize, width: usize| {
        let (qh, rh) = (height / n, height % n);
        let (qw, rw) = (width / n, width % n);

        qh * qw * imos[(n, n)] + qw * imos[(rh, n)] + qh * imos[(n, rw)] + imos[(rh, rw)]
    };

    let calc_rect_sum = |top: usize, bottom: usize, left: usize, right: usize| {
        calc_sum(top, left) + calc_sum(bottom, right)
            - (calc_sum(top, right) + calc_sum(bottom, left))
    };

    let ans = abcd
        .iter()
        .map(|&(a, b, c, d)| calc_rect_sum(a, c + 1, b, d + 1))
        .join("\n");
    println!("{}", ans);
}
