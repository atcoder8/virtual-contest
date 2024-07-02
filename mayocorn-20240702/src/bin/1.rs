use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    println!("{}", if [3, 5, 7].contains(&x) { "YES" } else { "NO" });
}
