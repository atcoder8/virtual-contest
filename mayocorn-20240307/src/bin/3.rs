use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (w, h, n): (usize, usize, usize),
        xya: [(usize, usize, usize); n],
    }

    let painted = |col: usize, row: usize| {
        xya.iter().any(|&(x, y, a)| match a {
            1 => col < x,
            2 => col >= x,
            3 => row < y,
            4 => row >= y,
            _ => unreachable!(),
        })
    };

    let ans = iproduct!(0..w, 0..h)
        .filter(|(col, row)| !painted(*col, *row))
        .count();
    println!("{}", ans);
}
