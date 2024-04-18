use itertools::Itertools;
use ndarray::{Array2, Axis};
use proconio::input;

const DIFFS: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];

fn main() {
    input! {
        n: usize,
    }

    let mut grid = Array2::from_elem((n, n), "".to_string());
    grid[(n / 2, n / 2)] = "T".to_string();

    let mut coord = (0, 0);
    let mut dir = 0;
    for i in 1..n.pow(2) {
        let (row, col) = coord;
        grid[(row, col)] = i.to_string();

        if i == n.pow(2) - 1 {
            break;
        }

        let progress = |dir: usize| {
            let (dr, dc) = DIFFS[dir];
            let nr = row.wrapping_add(dr);
            let nc = col.wrapping_add(dc);

            if nr >= n || nc >= n || !grid[(nr, nc)].is_empty() {
                return None;
            }

            Some((nr, nc))
        };

        coord = match progress(dir) {
            Some(next_coord) => next_coord,
            None => {
                dir = (dir + 1) % 4;
                progress(dir).unwrap()
            }
        };
    }

    println!(
        "{}",
        grid.axis_iter(Axis(1))
            .map(|line| line.iter().join(" "))
            .join("\n")
    );
}
