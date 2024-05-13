use std::collections::BinaryHeap;

use itertools::iproduct;
use ndarray::Array2;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n],
    }

    let start = iproduct!(0..h, 0..w)
        .find(|&(row, col)| aaa[row][col] == 'S')
        .unwrap();
    let goal = iproduct!(0..h, 0..w)
        .find(|&(row, col)| aaa[row][col] == 'T')
        .unwrap();

    let mut medicine_map = Array2::from_elem((h, w), 0);
    for &(r, c, e) in &rce {
        medicine_map[(r, c)] = e;
    }

    let mut energy_map = Array2::from_elem((h, w), 0);

    let mut heap = BinaryHeap::from([(medicine_map[start], start)]);
    while let Some((energy, coord)) = heap.pop() {
        if coord == goal {
            return true;
        }

        if energy <= energy_map[coord] {
            continue;
        }

        energy_map[coord] = energy;

        let (row, col) = coord;
        for (diff_row, diff_col) in DIFFS {
            let next_row = row.wrapping_add(diff_row);
            let next_col = col.wrapping_add(diff_col);
            let next_coord = (next_row, next_col);

            if next_row < h && next_col < w && aaa[next_row][next_col] != '#' {
                heap.push(((energy - 1).max(medicine_map[next_coord]), next_coord));
            }
        }
    }

    false
}
