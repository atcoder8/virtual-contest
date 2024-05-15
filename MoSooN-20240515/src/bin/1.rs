use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b, d): (usize, usize, usize),
    }

    let ans = (a..=b).step_by(d).join(" ");
    println!("{}", ans);
}
