use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut ans = 0;
    for ((x1, y1), (x2, y2), (x3, y3)) in xy.into_iter().tuple_combinations() {
        ans += ((x1 - x2) * (y1 - y3) != (x1 - x3) * (y1 - y2)) as usize;
    }
    println!("{}", ans);
}
