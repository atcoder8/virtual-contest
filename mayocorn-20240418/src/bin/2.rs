use proconio::{input, marker::Chars};
use regex::Regex;

fn main() {
    input! {
        mut s: String,
        t: Chars,
    }

    s.push('X');
    let s = s.to_uppercase();

    let re = Regex::new(&format!("{}.*{}.*{}", t[0], t[1], t[2])).unwrap();

    let ans = re.is_match(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}
