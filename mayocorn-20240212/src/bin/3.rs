use itertools::{izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let diff_len = s.len() - t.len();

    let mut diff = izip!(&s[diff_len..], &t)
        .filter(|(&c1, &c2)| !check_match(c1, c2))
        .count();

    let mut ans = vec![false; t.len() + 1];
    ans[0] = diff == 0;

    for i in 0..t.len() {
        if !check_match(s[i + diff_len], t[i]) {
            diff -= 1;
        }

        if !check_match(s[i], t[i]) {
            diff += 1;
        }

        ans[i + 1] = diff == 0;
    }

    println!(
        "{}",
        ans.iter().map(|&x| if x { "Yes" } else { "No" }).join("\n")
    );
}

fn check_match(c1: char, c2: char) -> bool {
    c1 == '?' || c2 == '?' || c1 == c2
}
