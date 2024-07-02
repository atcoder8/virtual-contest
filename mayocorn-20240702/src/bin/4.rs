use std::collections::VecDeque;

use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
    }

    let grid = Array2::from_shape_fn((h, w), |(row, col)| aaa[row][col]);

    let mut sight_map = Array2::from_elem((h, w), false);
    for coord in iproduct!(0..h, 0..w) {
        let Some(idx) = "^<>v".chars().position(|c| c == grid[coord]) else { continue; };
        let (dr, dc) = DIFFS[idx];

        let (mut row, mut col) = coord;
        loop {
            sight_map[(row, col)] = true;
            row = row.wrapping_add(dr);
            col = col.wrapping_add(dc);

            if row >= h || col >= w || grid[(row, col)] != '.' {
                break;
            }
        }
    }

    let find_coord = |symbol: char| {
        iproduct!(0..h, 0..w)
            .find(|&coord| grid[coord] == symbol)
            .unwrap()
    };

    let start_coord = find_coord('S');
    let goal_coord = find_coord('G');
    let mut dp = Array2::<Option<usize>>::from_elem((h, w), None);

    let mut queue = VecDeque::from([(start_coord, 0)]);
    while let Some((coord, cand_dist)) = queue.pop_front() {
        if dp[coord].is_some_and(|dist| cand_dist >= dist) {
            continue;
        }

        dp[coord] = Some(cand_dist);

        let (row, col) = coord;
        for (diff_row, diff_col) in DIFFS {
            let next_row = row.wrapping_add(diff_row);
            let next_col = col.wrapping_add(diff_col);
            let next_coord = (next_row, next_col);

            if next_row < h
                && next_col < w
                && !sight_map[next_coord]
                && ".G".contains(grid[next_coord])
            {
                queue.push_back((next_coord, cand_dist + 1));
            }
        }
    }

    match dp[goal_coord] {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
