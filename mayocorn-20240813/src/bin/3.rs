use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        tt: [usize; n],
    }

    let ans = tt.iter().fold(1_usize, |acc, t| acc.lcm(t));
    println!("{}", ans);
}
