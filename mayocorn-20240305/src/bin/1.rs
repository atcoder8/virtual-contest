use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s.chars().dedup().count();
    println!("{}", ans);
}
