use proconio::{input, marker::Chars};

fn main() {
    input! {
        name: Chars,
    }

    let n = name.len();
    let ans = (0..n / 2).all(|i| name[i] == name[n - 1 - i]);
    println!("{}", if ans { "YES" } else { "NO" });
}
