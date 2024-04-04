use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let ans = k * (k + 1) / 2 - aa.iter().filter(|&&a| a <= k).unique().sum::<usize>();
    println!("{}", ans);
}
