use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        abc: String,
    }

    let (a, b, c) = abc
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect_tuple()
        .unwrap();
    let ans = 100 * a + 10 * b + c + 100 * b + 10 * c + a + 100 * c + 10 * a + b;
    println!("{}", ans);
}
