use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        ccc: [[char; 4]; 4],
    }

    for cc in ccc.iter().rev() {
        println!("{}", cc.iter().rev().join(" "));
    }
}
