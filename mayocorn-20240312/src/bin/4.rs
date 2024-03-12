use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let sum_a = aa.iter().sum::<usize>();
    let x0 = sum_a - 2 * aa[1..].iter().step_by(2).sum::<usize>();

    let mut xx = vec![x0; n];
    for i in 1..n {
        xx[i] = 2 * aa[i - 1] - xx[i - 1];
    }

    println!("{}", xx.iter().join(" "));
}
