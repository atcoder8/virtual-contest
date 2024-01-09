use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (k, s): (usize, usize),
    }

    let ans = iproduct!(0..=k, 0..=k)
        .filter(|&(x, y)| x + y <= s && s - (x + y) <= k)
        .count();
    println!("{}", ans);
}
