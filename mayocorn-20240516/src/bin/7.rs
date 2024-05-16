use itertools::{enumerate, iproduct, Itertools};
use ndarray::Array2;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [[usize; w]; h],
    }

    let mut imos = Array2::from_shape_fn((h + 1, w + 1), |(row, col)| {
        if row == 0 || col == 0 {
            return 0;
        }

        aaa[row - 1][col - 1]
    });
    for (row, col) in iproduct!(0..=h, 0..w) {
        imos[(row, col + 1)] += imos[(row, col)];
    }
    for (row, col) in iproduct!(0..h, 0..=w) {
        imos[(row + 1, col)] += imos[(row, col)];
    }

    let calc_rect_sum = |top: usize, bottom: usize, left: usize, right: usize| {
        let inclusion = imos[(top, left)] + imos[(bottom, right)];
        let exclusion = imos[(top, right)] + imos[(bottom, left)];

        inclusion - exclusion
    };

    let find_bounds = |aa: &[usize], lower_limit: usize| {
        let mut bounds = (0..w).collect_vec();

        for (col, &a) in enumerate(aa).rev() {
            if a < lower_limit {
                continue;
            }

            if col < w - 1 && bounds[col + 1] > col + 1 {
                bounds[col] = bounds[col + 1];
            } else {
                bounds[col] = col + 1;
            }
        }

        bounds
    };

    let max_val = *aaa.iter().flatten().max().unwrap();

    let mut ans = 0;

    for lower_limit in 1..=max_val {
        let bounds_each_row = aaa
            .iter()
            .map(|aa| find_bounds(aa, lower_limit))
            .collect_vec();

        for left in 0..w {
            let mut rows_each_lengths = vec![vec![]; w - left + 1];
            for (row, bounds) in enumerate(&bounds_each_row) {
                rows_each_lengths[bounds[left] - left].push(row);
            }

            let mut ends: Vec<Option<(usize, usize)>> = vec![None; h];

            for (length, rows) in enumerate(&rows_each_lengths).rev() {
                for &row in rows {
                    let top = row
                        .checked_sub(1)
                        .and_then(|adj| ends[adj].map(|end| end.0))
                        .unwrap_or(row);
                    let bottom = ends
                        .get(row + 1)
                        .and_then(|end| end.map(|end| end.1))
                        .unwrap_or(row + 1);

                    ends[row] = Some((top, bottom));

                    ends[top].as_mut().unwrap().1 = bottom;
                    ends[bottom - 1].as_mut().unwrap().0 = top;

                    let right = left + length;
                    let score = lower_limit * calc_rect_sum(top, bottom, left, right);
                    ans = ans.max(score);
                }
            }
        }
    }

    println!("{}", ans);
}
