use itertools::Itertools;
use ndarray::Array2;
use proconio::input;

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
    }

    let dirs = [(h - 1, 0), (0, 1), (1, 0), (0, w - 1)];

    let mut grid = Array2::from_elem((h, w), false);

    let mut dir = 0;
    let mut coord = (0, 0);
    for _ in 0..n {
        grid[coord] = !grid[coord];

        if grid[coord] {
            dir = (dir + 1) % 4;
        } else {
            dir = (dir + 3) % 4;
        }

        let (diff_row, diff_col) = dirs[dir];
        coord = ((coord.0 + diff_row) % h, (coord.1 + diff_col) % w);
    }

    let ans = (0..h)
        .map(|row| {
            (0..w)
                .map(|col| if grid[(row, col)] { '#' } else { '.' })
                .join("")
        })
        .join("\n");
    println!("{}", ans);
}
