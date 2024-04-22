use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    let ans = (0..n)
        .sorted_unstable_by_key(|&i| aa[i])
        .nth(n - 2)
        .unwrap()
        + 1;
    println!("{}", ans);
}
