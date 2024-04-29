use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s.chars().filter(|&c| !"aeiou".contains(c)).join("");
    println!("{}", ans);
}
