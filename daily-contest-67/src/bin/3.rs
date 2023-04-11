use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, query_num): (usize, usize),
        xy: [(i64, i64); n],
        qq: [Usize1; query_num],
    }

    let ab = xy.iter().map(|&(x, y)| (x - y, x + y)).collect_vec();
    let min_a = ab.iter().map(|x| x.0).min().unwrap();
    let max_a = ab.iter().map(|x| x.0).max().unwrap();
    let min_b = ab.iter().map(|x| x.1).min().unwrap();
    let max_b = ab.iter().map(|x| x.1).max().unwrap();

    for &q in &qq {
        let (a, b) = ab[q];

        let ans = (a - min_a).max(max_a - a).max(b - min_b).max(max_b - b);
        println!("{}", ans);
    }
}
