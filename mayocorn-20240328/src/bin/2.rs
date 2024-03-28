use std::str::FromStr;

use num::BigUint;
use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }

    let a = BigUint::from_str(&a).unwrap();
    let b = BigUint::from_str(&b).unwrap();
    let ans = match a.cmp(&b) {
        std::cmp::Ordering::Less => "LESS",
        std::cmp::Ordering::Equal => "EQUAL",
        std::cmp::Ordering::Greater => "GREATER",
    };
    println!("{}", ans);
}
