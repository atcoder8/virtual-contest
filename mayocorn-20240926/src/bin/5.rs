use itertools::iproduct;
use ndarray::Array2;
use proconio::input;

const HEIGHT: usize = 2;
const WIDTH: usize = 4;

const AREAS: [[usize; WIDTH]; HEIGHT] = [[2, 1, 0, 1], [1, 2, 1, 0]];

const OFFSET: i64 = 10_i64.pow(9);

fn main() {
    input! {
        corner: [i64; 4],
    }

    let mut acc_mat = Array2::from_shape_fn((HEIGHT + 1, WIDTH + 1), |(row, col)| {
        if row >= 1 && col >= 1 {
            AREAS[row - 1][col - 1]
        } else {
            0
        }
    });
    for (row, col) in iproduct!(1..HEIGHT, 1..=WIDTH) {
        acc_mat[(row + 1, col)] += acc_mat[(row, col)];
    }
    for (row, col) in iproduct!(1..=HEIGHT, 1..WIDTH) {
        acc_mat[(row, col + 1)] += acc_mat[(row, col)];
    }

    let calc_cornered_rect = |height: usize, width: usize| {
        let (qh, rh) = (height / HEIGHT, height % HEIGHT);
        let (qw, rw) = (width / WIDTH, width % WIDTH);

        qh * qw * acc_mat[(HEIGHT, WIDTH)]
            + acc_mat[(rh, WIDTH)] * qw
            + acc_mat[(HEIGHT, rw)] * qh
            + acc_mat[(rh, rw)]
    };

    let calc_rect = |top: usize, bottom: usize, left: usize, right: usize| {
        let inclusive = calc_cornered_rect(top, left) + calc_cornered_rect(bottom, right);
        let exclusive = calc_cornered_rect(top, right) + calc_cornered_rect(bottom, left);
        inclusive - exclusive
    };

    let [left, top, right, bottom] = std::array::from_fn(|i| (corner[i] + OFFSET) as usize);
    println!("{}", calc_rect(top, bottom, left, right));
}
