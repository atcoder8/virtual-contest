use std::cmp::min_by_key;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let cand1 = n / 5 * 5;
    let cand2 = cand1 + 5;

    let ans = min_by_key(cand1, cand2, |&coord| n.abs_diff(coord));
    println!("{}", ans);
}
