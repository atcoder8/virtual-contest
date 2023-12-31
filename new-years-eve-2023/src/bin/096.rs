use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    }

    let ans = n % 500 <= a;
    println!("{}", if ans { "Yes" } else { "No" });
}
