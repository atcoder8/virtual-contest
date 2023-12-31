use proconio::input;

fn main() {
    input! {
        x: char,
        mut s: String,
    }

    s.retain(|c| c != x);

    println!("{}", s);
}
