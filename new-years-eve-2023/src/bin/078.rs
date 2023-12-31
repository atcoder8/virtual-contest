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
        mut ss: [Chars; h],
    }

    for (row, col) in iproduct!(0..h, 0..w) {
        if ss[row][col] != '.' {
            continue;
        }

        let mut bomb_num = 0;

        for (diff_row, diff_col) in DIFFS {
            let (next_row, next_col) = (row.wrapping_add(diff_row), col.wrapping_add(diff_col));

            if next_row >= h || next_col >= w {
                continue;
            }

            if ss[next_row][next_col] == '#' {
                bomb_num += 1;
            }
        }

        ss[row][col] = char::from_digit(bomb_num, 10).unwrap();
    }

    for s in &ss {
        println!("{}", s.iter().join(""));
    }
}
