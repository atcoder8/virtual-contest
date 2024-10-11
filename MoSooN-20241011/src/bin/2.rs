use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let positions = s.chars().positions(|c| c == '|').collect_vec();
    let ans = s[..positions[0]].to_string() + &s[positions[1] + 1..];
    println!("{}", ans);
}
