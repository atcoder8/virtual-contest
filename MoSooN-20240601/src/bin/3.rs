use itertools::{iproduct, Itertools};
use ndarray::{s, Array2, Axis};
use proconio::input;

const N: usize = 9;

fn main() {
    input! {
        aaa: [usize; N.pow(2)],
    }

    let grid = Array2::from_shape_vec((N, N), aaa).unwrap();

    let hor = grid.axis_iter(Axis(0)).all(|line| line.iter().all_unique());
    let ver = grid.axis_iter(Axis(1)).all(|line| line.iter().all_unique());
    let block = iproduct!((0..N).step_by(3), (0..N).step_by(3)).all(|(top, left)| {
        grid.slice(s![top..top + 3, left..left + 3])
            .iter()
            .all_unique()
    });

    let ans = hor && ver && block;
    println!("{}", if ans { "Yes" } else { "No" });
}
