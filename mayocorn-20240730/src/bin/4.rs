use ndarray::prelude::*;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let mut visited_matrix = Array2::from_elem((n, m), false);
    let mut used_matrix = Array2::from_elem((n, m), false);

    let mut stack = vec![(1, 1)];
    while let Some(coord) = stack.pop() {
        if used_matrix[coord] {
            continue;
        }

        used_matrix[coord] = true;
        visited_matrix[coord] = true;

        for (diff_row, diff_col) in DIFFS {
            let (mut next_row, mut next_col) = coord;
            loop {
                let adj_row = next_row.wrapping_add(diff_row);
                let adj_col = next_col.wrapping_add(diff_col);

                if ss[adj_row][adj_col] == '#' {
                    break;
                }

                next_row = adj_row;
                next_col = adj_col;

                visited_matrix[(next_row, next_col)] = true;
            }

            stack.push((next_row, next_col));
        }
    }

    let visited_num = visited_matrix.iter().filter(|&&visited| visited).count();
    println!("{}", visited_num);
}
