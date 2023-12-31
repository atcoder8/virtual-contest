use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        w: String,
    }

    let ans = w
        .chars()
        .sorted_unstable()
        .dedup_with_count()
        .all(|(cnt, _)| cnt % 2 == 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
