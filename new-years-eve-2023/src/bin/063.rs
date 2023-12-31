use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (k, x): (i64, i64),
    }

    println!("{}", (x - k + 1..=x + k - 1).join(" "));
}
