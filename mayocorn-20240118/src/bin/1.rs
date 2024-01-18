use proconio::input;

fn main() {
    input! {
        (a, b, c): (i64, i64, i64),
    }

    let ans = b - a == c - b;
    println!("{}", if ans { "YES" } else { "NO" });
}
