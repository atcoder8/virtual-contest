use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = !s.chars().all_equal();
    println!("{}", if ans { "Yes" } else { "No" });
}
