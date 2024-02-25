use std::collections::VecDeque;

use itertools::{enumerate, Itertools};
use ndarray::Array2;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

const DIRS: [(usize, usize); 4] = [(!0, !0), (!0, 1), (1, !0), (1, 1)];

fn main() {
    input! {
        n: usize,
        start_coord: (Usize1, Usize1),
        goal_coord: (Usize1, Usize1),
        ss: [Chars; n],
    }

    let mut dp = Array2::<[Option<usize>; 4]>::from_elem((n, n), [None; 4]);
    let init = (0..4).map(|dir| (start_coord, dir)).collect_vec();
    let mut queue = VecDeque::from(init);
    dp[start_coord] = [Some(0); 4];
    while let Some((coord, dir)) = queue.pop_front() {
        let dist = dp[coord][dir].unwrap();

        let (row, col) = coord;
        for (next_dir, (diff_row, diff_col)) in enumerate(DIRS) {
            let mut next_row = row;
            let mut next_col = col;

            loop {
                next_row = next_row.wrapping_add(diff_row);
                next_col = next_col.wrapping_add(diff_col);

                if next_row >= n || next_col >= n || ss[next_row][next_col] == '#' {
                    break;
                }

                let next_coord = (next_row, next_col);

                if dp[next_coord][next_dir].is_some() {
                    break;
                }

                dp[next_coord][next_dir] = Some(dist + 1);

                queue.push_back((next_coord, next_dir));
            }
        }
    }

    let ans = dp[goal_coord].iter().filter_map(|&dist| dist).min();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
