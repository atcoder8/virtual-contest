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

    let mut s_idx = 0_usize;
    let mut stack = vec![];
    for &c in &t {
        if s_idx < s.len() && s[s_idx] == c {
            stack.push(c);
            s_idx += 1;
            continue;
        }

        if stack.len() < 2 || stack[stack.len() - 1] != c || stack[stack.len() - 2] != c {
            return false;
        }

        stack.push(c);
    }

    s_idx == s.len()
}
