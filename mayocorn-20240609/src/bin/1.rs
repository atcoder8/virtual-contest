use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let ans = iproduct!(0..n, 0..n)
        .map(|(i, j)| {
            if i == j {
                ab[i].0 + ab[i].1
            } else {
                ab[i].0.max(ab[j].1)
            }
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
