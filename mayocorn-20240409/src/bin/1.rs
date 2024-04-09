use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"^[A-Z][a-z]*$").unwrap();
    println!("{}", if re.is_match(&s) { "Yes" } else { "No" });
}
