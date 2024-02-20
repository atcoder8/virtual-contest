use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}{}", &s[1..], &s[..1]);
}
