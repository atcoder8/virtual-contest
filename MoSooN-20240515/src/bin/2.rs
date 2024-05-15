use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    }

    println!("{}", if a >= n % 500 { "Yes" } else { "No" });
}
