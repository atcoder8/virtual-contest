use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let half = (0..s.len() / 2)
        .rev()
        .find(|&half| s[..half] == s[half..2 * half])
        .unwrap();

    println!("{}", 2 * half);
}
