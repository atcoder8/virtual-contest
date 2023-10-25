use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let orders = aa
        .iter()
        .enumerate()
        .sorted_by_key(|&(i, &a)| (a, Reverse(i)))
        .map(|(i, _)| i);

    let mut ans = None;
    let mut max_left = None;
    for i in orders {
        if i < k {
            if max_left.is_none() || i > max_left.unwrap() {
                max_left = Some(i);
            }
        } else {
            if let Some(max_left) = max_left {
                if ans.is_none() || i - max_left < ans.unwrap() {
                    ans = Some(i - max_left);
                }
            }
        }
    }

    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
