use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let xor = aa.iter().fold(0_usize, |acc, x| acc ^ x);
    let ans = aa.iter().map(|a| xor ^ a).join(" ");
    println!("{}", ans);
}
