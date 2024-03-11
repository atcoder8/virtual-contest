use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k, x): (usize, usize, usize),
        aa: [usize; n],
    }

    let mut rem_coupon = k;
    let mut rem_prices = aa
        .iter()
        .map(|&a| {
            let use_coupon = (a / x).min(rem_coupon);
            rem_coupon -= use_coupon;

            a - x * use_coupon
        })
        .collect_vec();
    rem_prices.sort_unstable_by_key(|&rem_price| Reverse(rem_price));

    let ans = rem_prices
        .iter()
        .map(|&rem_price| {
            if rem_coupon != 0 {
                rem_coupon -= 1;

                0
            } else {
                rem_price
            }
        })
        .sum::<usize>();
    println!("{}", ans);
}
