use std::collections::VecDeque;

use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let mut dist_mat = Array2::<Option<usize>>::from_elem((h, w), None);
    let mut queue = VecDeque::from([((0, 0), 0)]);
    while let Some((coord, dist)) = queue.pop_front() {
        if dist_mat[coord].is_some() {
            continue;
        }

        dist_mat[coord] = Some(dist);

        for (diff_row, diff_col) in DIFFS {
            let next_row = coord.0.wrapping_add(diff_row);
            let next_col = coord.1.wrapping_add(diff_col);

            if next_row < h && next_col < w && sss[next_row][next_col] == '.' {
                queue.push_front(((next_row, next_col), dist));
            }
        }

        for (diff_row, diff_col) in iproduct!(0..=4, 0..=4) {
            if (diff_row == 0 || diff_row == 4) && (diff_col == 0 || diff_col == 4) {
                continue;
            }

            let next_row = (coord.0 + diff_row).wrapping_sub(2);
            let next_col = (coord.1 + diff_col).wrapping_sub(2);

            if next_row < h && next_col < w {
                queue.push_back(((next_row, next_col), dist + 1));
            }
        }
    }

    println!("{}", dist_mat[(h - 1, w - 1)].unwrap());
}
