use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "AC" } else { "WA" });
}

fn solve() -> bool {
    input! {
        s: Chars,
    }

    if s[0] != 'A' {
        return false;
    }

    if s[2..s.len() - 1].iter().filter(|&&c| c == 'C').count() != 1 {
        return false;
    }

    s.iter().filter(|c| c.is_ascii_uppercase()).count() == 2
}
