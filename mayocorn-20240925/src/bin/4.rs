use itertools::{enumerate, iproduct};
use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let moveable = |coord: (usize, usize)| {
        let (row, col) = coord;
        for (diff_row, diff_col) in DIFFS {
            let adj_row = row.wrapping_add(diff_row);
            let adj_col = col.wrapping_add(diff_col);

            if adj_row < h && adj_col < w && ss[adj_row][adj_col] == '#' {
                return false;
            }
        }

        true
    };

    let mut max_degree = 0_usize;
    let mut visited_core = Array2::from_elem((h, w), false);
    let mut labels = Array2::<Option<usize>>::from_elem((h, w), None);
    for (label, start_coord) in enumerate(iproduct!(0..h, 0..w)) {
        if visited_core[start_coord] || ss[start_coord.0][start_coord.1] == '#' {
            continue;
        }

        let mut degree = 0_usize;
        let mut stack = vec![start_coord];
        while let Some(coord) = stack.pop() {
            if visited_core[coord] {
                continue;
            }

            match labels[coord] {
                Some(border_label) if border_label == label => {}
                _ => {
                    degree += 1;
                    labels[coord] = Some(label)
                }
            }

            if !moveable(coord) {
                continue;
            }

            visited_core[coord] = true;

            let (row, col) = coord;
            for (diff_row, diff_col) in DIFFS {
                let adj_row = row.wrapping_add(diff_row);
                let adj_col = col.wrapping_add(diff_col);
                let adj_coord = (adj_row, adj_col);

                if adj_row < h && adj_col < w {
                    stack.push(adj_coord);
                }
            }
        }

        max_degree = max_degree.max(degree);
    }

    println!("{}", max_degree);
}
