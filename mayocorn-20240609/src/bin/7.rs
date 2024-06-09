use itertools::{enumerate, iproduct};
use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        abcd: [(Usize1, Usize1, usize, usize); n],
    }

    let mut square_to_cand_pieces = Array2::from_elem((h, w), vec![]);
    for (i, &(a, b, c, d)) in enumerate(&abcd) {
        for coord in iproduct!(a..c, b..d) {
            square_to_cand_pieces[coord].push(i);
        }
    }
}
