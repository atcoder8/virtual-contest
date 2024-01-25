use itertools::{izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: Chars,
        t: Chars,
    }

    if s.iter().unique().count() != t.iter().unique().count() {
        return false;
    }

    let mut decided: Vec<Option<usize>> = vec![None; 26];
    for (&c1, &c2) in izip!(&s, &t) {
        let u1 = char_to_usize(c1);
        let u2 = char_to_usize(c2);

        if decided[u1].is_some_and(|dest| dest != u2) {
            return false;
        }

        decided[u1] = Some(u2);
    }

    true
}

fn char_to_usize(c: char) -> usize {
    (c as u8 - b'a') as usize
}
