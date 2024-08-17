use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = (0..s.len() / 2).all(|i| is_match(s[i], s[s.len() - 1 - i]));
    println!("{}", if ans { "YES" } else { "NO" });
}

fn is_match(c1: char, c2: char) -> bool {
    c1 == '*' || c2 == '*' || c1 == c2
}
