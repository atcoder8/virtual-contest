use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let ans = iproduct!(0..n, 0..n)
        .map(|(i, j)| {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];

            (x1 - x2).hypot(y1 - y2)
        })
        .sum::<f64>()
        / n as f64;
    println!("{}", ans);
}
