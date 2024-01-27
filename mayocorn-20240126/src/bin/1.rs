use itertools::enumerate;
use proconio::{input, marker::Chars};

const ACGT: &str = "ACGT";

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut left = 0;
    for (right, c) in enumerate(s) {
        if ACGT.contains(c) {
            ans = ans.max(right - left + 1);
        } else {
            left = right + 1;
        }
    }

    println!("{}", ans);
}
