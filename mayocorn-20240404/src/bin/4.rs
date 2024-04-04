use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        grid: [Chars; h],
    }

    let mut ans = 0_usize;
    for (row, col) in iproduct!(0..h - 1, 0..w - 1) {
        let cnt = iproduct!(0..2, 0..2)
            .filter(|(dx, dy)| grid[row + dx][col + dy] == '#')
            .count();
        ans += (cnt == 1 || cnt == 3) as usize;
    }
    println!("{}", ans);
}
