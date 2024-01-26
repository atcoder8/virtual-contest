use proconio::{input, marker::Chars};

const ACGT: &str = "ACGT";

fn main() {
    input! {
        s: Chars,
    }

    let is_ok = |seq: &[char]| seq.iter().all(|&c| ACGT.contains(c));

    let n = s.len();

    let mut ans = 0;
    for left in 0..n {
        for right in left + 1..=n {
            if is_ok(&s[left..right]) {
                ans = ans.max(right - left);
            }
        }
    }

    println!("{}", ans);
}
