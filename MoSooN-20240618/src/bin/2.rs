use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let len = s.len();
    println!("{}{}{}", s[0], s.len() - 2, s[len - 1]);
}
