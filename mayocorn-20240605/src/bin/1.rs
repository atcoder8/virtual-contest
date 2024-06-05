use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, w): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by_key(|v| Reverse(v.0));

    let mut ans = 0;
    let mut rem = w;
    for &(a, b) in &ab {
        let weight = b.min(rem);
        ans += a * weight;
        rem -= weight;
    }

    println!("{}", ans);
}
