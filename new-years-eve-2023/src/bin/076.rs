use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s[..1].to_uppercase() + &s[1..].to_lowercase();
    println!("{}", ans);
}
