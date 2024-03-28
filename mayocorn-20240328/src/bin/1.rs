use itertools::iproduct;
use ndarray::Array2;
use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    let mut grid = Array2::from_elem((a * n, b * n), false);
    for (tile_row, tile_col) in iproduct!(0..n, 0..n) {
        if (tile_row + tile_col) % 2 == 0 {
            continue;
        }

        for (i, j) in iproduct!(0..a, 0..b) {
            grid[(a * tile_row + i, b * tile_col + j)] = true;
        }
    }

    for row in 0..a * n {
        for col in 0..b * n {
            print!("{}", if grid[(row, col)] { '#' } else { '.' });
        }
        println!();
    }
}
