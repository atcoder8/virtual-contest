use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        x: String,
    }

    let mut prefix_sum = vec![0; x.len() + 1];
    for (i, c) in enumerate(x.chars()) {
        prefix_sum[i + 1] += prefix_sum[i] + c.to_digit(10).unwrap() as usize;
    }

    for i in (0..x.len()).rev() {
        prefix_sum[i] += prefix_sum[i + 1] / 10;
        prefix_sum[i + 1] %= 10;
    }

    let mut ans = prefix_sum.iter().join("");
    if &ans[..1] == "0" {
        ans.remove(0);
    }

    println!("{}", ans);
}
