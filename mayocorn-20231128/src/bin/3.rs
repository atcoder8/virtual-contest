use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    let sum_aoki: usize = ab.iter().map(|x| x.0).sum();

    ab.sort_unstable_by_key(|&(a, b)| Reverse(2 * a + b));

    let mut ans = 0;
    let mut sum = 0;
    for &(a, b) in &ab {
        if sum > sum_aoki {
            break;
        }

        ans += 1;
        sum += 2 * a + b;
    }

    println!("{}", ans);
}
