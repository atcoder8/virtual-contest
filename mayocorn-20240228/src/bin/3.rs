use itertools::{iproduct, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        mut grid: [Chars; h],
    }

    let mut hor_black = vec![false; h];
    let mut ver_black = vec![false; w];
    for (row, col) in iproduct!(0..h, 0..w) {
        if grid[row][col] == '#' {
            hor_black[row] = true;
            ver_black[col] = true;
        }
    }

    let rem_rows = (0..h).filter(|&row| hor_black[row]).collect_vec();
    let rem_cols = (0..w).filter(|&col| ver_black[col]).collect_vec();

    let ans = rem_rows
        .iter()
        .map(|&row| rem_cols.iter().map(|&col| grid[row][col]).join(""))
        .join("\n");
    println!("{}", ans);
}
