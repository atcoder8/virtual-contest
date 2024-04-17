use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: Chars,
        t: Chars,
    }

    if s.len() > t.len() {
        return false;
    }

    let mut idx = 0;
    for &c in &t {
        if idx < s.len() && s[idx] == c {
            idx += 1;
        } else if idx < 2 || s[idx - 1] != s[idx - 2] || s[idx - 1] != c {
            return false;
        }
    }

    idx == s.len()
}
