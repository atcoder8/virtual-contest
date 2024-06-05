use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let u = xy.iter().map(|&(x, y)| x - y).collect_vec();
    let v = xy.iter().map(|&(x, y)| x + y).collect_vec();

    let diff_u = u.iter().max().unwrap() - u.iter().min().unwrap();
    let diff_v = v.iter().max().unwrap() - v.iter().min().unwrap();

    println!("{}", diff_u.max(diff_v));
}
