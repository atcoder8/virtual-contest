use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = c % a.gcd(&b) == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
