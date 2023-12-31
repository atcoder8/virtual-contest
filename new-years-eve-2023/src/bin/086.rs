use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let ans = ss.iter().unique().count();
    println!("{}", ans);
}
