use itertools::{enumerate, iproduct, Itertools};
use ndarray::Array2;
use proconio::{input, marker::Chars};

const LEN: usize = 4;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        pppp: [[Chars; LEN]; 3],
    }

    let area = pppp
        .iter()
        .flatten()
        .flatten()
        .filter(|&&p| p == '#')
        .count();

    if area != LEN.pow(2) {
        return false;
    }

    let polyominoes = pppp
        .iter()
        .map(|ppp| {
            let polyomino = Array2::from_shape_fn((LEN, LEN), |(row, col)| ppp[row][col] == '#');
            aligned(&polyomino)
        })
        .collect_vec();

    let rotated_each_polyomino = polyominoes
        .iter()
        .map(|polyomino| {
            let mut rotated_vec = vec![aligned(polyomino)];
            for i in 1..4 {
                let rotated = rotated(&rotated_vec[i - 1]);
                rotated_vec.push(rotated);
            }

            rotated_vec
        })
        .collect_vec();

    let is_ok = |rotates: usize, shifts: usize| {
        let mut grid = Array2::from_elem((LEN, LEN), false);

        for (i, rotated_vec) in enumerate(&rotated_each_polyomino) {
            let rotate_num = rotates / 4_usize.pow(i as u32) % 4;
            let shift = shifts / LEN.pow(2).pow(i as u32) % LEN.pow(2);
            let (shift_row, shift_col) = (shift / LEN, shift % LEN);

            let rotated = &rotated_vec[rotate_num];
            for (row, col) in iproduct!(0..LEN, 0..LEN) {
                if !rotated[(row, col)] {
                    continue;
                }

                let shifted_row = row + shift_row;
                let shifted_col = col + shift_col;

                if shifted_row >= LEN || shifted_col >= LEN || grid[(shifted_row, shifted_col)] {
                    return false;
                }

                grid[(shifted_row, shifted_col)] = true;
            }
        }

        true
    };

    iproduct!(0..64, 0..LEN.pow(6)).any(|(rotates, shifts)| is_ok(rotates, shifts))
}

fn aligned(polyomino: &Array2<bool>) -> Array2<bool> {
    let top = (0..LEN)
        .find(|&row| (0..LEN).any(|col| polyomino[(row, col)]))
        .unwrap();
    let left = (0..LEN)
        .find(|&col| (0..LEN).any(|row| polyomino[(row, col)]))
        .unwrap();

    let mut shifted = Array2::from_elem((LEN, LEN), false);
    for (row, col) in iproduct!(top..LEN, left..LEN) {
        shifted[(row - top, col - left)] = polyomino[(row, col)];
    }

    shifted
}

fn rotated(polyomino: &Array2<bool>) -> Array2<bool> {
    let mut rotated = Array2::from_elem((LEN, LEN), false);
    for (row, col) in iproduct!(0..4, 0..4) {
        rotated[(LEN - 1 - col, row)] = polyomino[(row, col)];
    }

    aligned(&rotated)
}
