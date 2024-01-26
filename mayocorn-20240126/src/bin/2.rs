use itertools::{iproduct, Itertools};
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 8] = [
    (!0, !0),
    (!0, 0),
    (!0, 1),
    (0, !0),
    (0, 1),
    (1, !0),
    (1, 0),
    (1, 1),
];

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let mut grid = ss.clone();
    for (row, col) in iproduct!(0..h, 0..w) {
        if ss[row][col] == '#' {
            continue;
        }

        let mut cnt = 0;
        for (diff_row, diff_col) in DIFFS {
            let (next_row, next_col) = (row.wrapping_add(diff_row), col.wrapping_add(diff_col));

            if next_row >= h || next_col >= w {
                continue;
            }

            cnt += (ss[next_row][next_col] == '#') as u32;
        }

        grid[row][col] = char::from_digit(cnt, 10).unwrap();
    }

    println!(
        "{}",
        grid.iter().map(|line| line.iter().join("")).join("\n")
    );
}
