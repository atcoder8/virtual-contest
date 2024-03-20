use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: Chars,
    }

    let mut addable = true;
    let mut l = 0;
    let mut r = s.len();
    while l < r {
        r -= 1;

        if s[r] != 'a' {
            addable = false;
        }

        if s[l] == s[r] {
            l += 1;
        } else {
            if !addable {
                return false;
            }
        }
    }

    true
}
