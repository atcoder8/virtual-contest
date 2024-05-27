use itertools::{izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut match_flags = vec![false; t.len() + 1];

    let mut diff = izip!(&s[s.len() - t.len()..], &t)
        .filter(|&(&c1, &c2)| !is_match(c1, c2))
        .count();
    match_flags[0] = diff == 0;

    for x in 1..=t.len() {
        diff -= !is_match(s[s.len() - t.len() + x - 1], t[x - 1]) as usize;
        diff += !is_match(s[x - 1], t[x - 1]) as usize;
        match_flags[x] = diff == 0;
    }

    let ans = match_flags
        .iter()
        .map(|&flag| if flag { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}

fn is_match(c1: char, c2: char) -> bool {
    c1 == '?' || c2 == '?' || c1 == c2
}
