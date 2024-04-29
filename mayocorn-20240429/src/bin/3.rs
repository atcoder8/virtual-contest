use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

const N: usize = 9;

fn main() {
    input! {
        grid: [Chars; N],
    }

    let grid = Array2::from_shape_fn((N, N), |(row, col)| grid[row][col] == '#');

    let is_ok = |p0: (usize, usize), diff: (usize, usize)| {
        if !grid[p0] {
            return false;
        }

        let (mut x, mut y) = p0;
        let (mut dx, mut dy) = diff;
        for _ in 0..3 {
            (x, y) = (x.wrapping_add(dx), y.wrapping_add(dy));

            if x >= N || y >= N || !grid[(x, y)] {
                return false;
            }

            (dx, dy) = next_diff((dx, dy));
        }

        true
    };

    let mut ans = 0;
    for (x0, y0) in iproduct!(0..N, 0..N) {
        for diff in iproduct!(1..=N - 1 - x0, 0..=N - 1 - y0) {
            ans += is_ok((x0, y0), diff) as usize;
        }
    }
    println!("{}", ans);
}

fn next_diff(diff: (usize, usize)) -> (usize, usize) {
    (diff.1.wrapping_neg(), diff.0)
}
