use proconio::input;

fn main() {
    input! {
        (m, d): (usize, usize),
    }

    let ans = m % d == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
