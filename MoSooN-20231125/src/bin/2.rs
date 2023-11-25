use proconio::{input, marker::Usize1};

const N: usize = 15;

fn main() {
    input! {
        (r, c): (Usize1, Usize1),
    }

    let ans = r.min(N - 1 - r).min(c).min(N - 1 - c) % 2 == 0;
    println!("{}", if ans { "black" } else { "white" });
}
