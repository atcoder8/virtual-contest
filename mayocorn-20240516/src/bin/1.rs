use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let is_ok = |s1: &str, s2: &str| {
        let combined = (s1.to_owned() + s2).chars().collect_vec();
        (0..combined.len() / 2).all(|i| combined[i] == combined[combined.len() - 1 - i])
    };

    let ans = ss
        .iter()
        .permutations(2)
        .any(|pair| is_ok(pair[0], pair[1]));
    println!("{}", if ans { "Yes" } else { "No" });
}
