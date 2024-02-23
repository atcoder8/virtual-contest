use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let ans = s[n - 1] == 'o';
    println!("{}", if ans { "Yes" } else { "No" });
}
