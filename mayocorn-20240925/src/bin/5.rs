use itertools::{iproduct, Itertools};
use ndarray::Array2;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w, k): (usize, usize, usize),
        sss: [Chars; h],
    }

    let mut prefix_sum = Array2::from_shape_fn((h + 1, w + 1), |(row, col)| {
        if row >= 1 && col >= 1 {
            (sss[row - 1][col - 1] == '1') as usize
        } else {
            0
        }
    });
    for (row, col) in iproduct!(1..h, 1..=w) {
        prefix_sum[(row + 1, col)] += prefix_sum[(row, col)];
    }
    for (row, col) in iproduct!(1..=h, 1..w) {
        prefix_sum[(row, col + 1)] += prefix_sum[(row, col)];
    }

    let calc_rect_sum = |top: usize, bottom: usize, left: usize, right: usize| {
        let inclusive = prefix_sum[(top, left)] + prefix_sum[(bottom, right)];
        let exclusive = prefix_sum[(top, right)] + prefix_sum[(bottom, left)];
        inclusive - exclusive
    };

    let count_divide_vertically = |bits: usize| {
        let mut cnt = 0_usize;

        let mut borders = (1..h).filter(|i| bits >> (i - 1) & 1 == 1).collect_vec();
        borders.insert(0, 0);
        borders.push(h);

        let mut left = 0;
        loop {
            let is_ok = |right: usize| {
                borders
                    .iter()
                    .tuple_windows()
                    .all(|(&top, &bottom)| calc_rect_sum(top, bottom, left, right) <= k)
            };

            let mut right = left;
            while right < w && is_ok(right + 1) {
                right += 1;
            }

            if right == left {
                return None;
            }

            if right == w {
                return Some(cnt);
            }

            cnt += 1;
            left = right;
        }
    };

    let ans = (0_usize..1 << (h - 1))
        .filter_map(|bits| Some(bits.count_ones() as usize + count_divide_vertically(bits)?))
        .min()
        .unwrap();
    println!("{}", ans);
}
