use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    input! {
        _w: usize,
    }

    let ans = chain!(
        1_usize..=99,
        (100..=9900).step_by(100),
        (10000..=990000).step_by(10000)
    )
    .join(" ");
    println!("{}\n{}", 99 * 3, ans);
}
