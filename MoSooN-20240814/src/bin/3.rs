use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }

    s.sort_unstable();
    t.sort_unstable_by_key(|&ch| Reverse(ch));

    println!("{}", if s < t { "Yes" } else { "No" });
}
