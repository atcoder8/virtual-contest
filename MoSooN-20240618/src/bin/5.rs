use itertools::Itertools;
use ndarray::{Array2, Axis};
use proconio::input;

const N: usize = 100;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let mut grid = Array2::from_shape_fn((100, 100), |(row, _)| row < N / 2);

    let mut row = 0;
    let mut col = 0;
    for _ in 0..a - 1 {
        grid[(row, col)] = false;

        if col + 2 < N {
            col += 2;
        } else {
            row += 2;
            col = 0;
        }
    }

    let mut row = N - 1;
    let mut col = 0;
    for _ in 0..b - 1 {
        grid[(row, col)] = true;

        if col + 2 < N {
            col += 2;
        } else {
            row -= 2;
            col = 0;
        }
    }

    let ans = grid
        .axis_iter(Axis(0))
        .map(|line| {
            line.iter()
                .map(|&color| if color { "#" } else { "." })
                .join("")
        })
        .join("\n");
    println!("{} {}\n{}", N, N, ans);
}
