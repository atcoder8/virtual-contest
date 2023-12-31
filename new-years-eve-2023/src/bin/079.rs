use proconio::input;

fn main() {
    input! {
        (_, s, _): (String, String, String),
    }

    println!("A{}C", &s[..1]);
}
