use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        colors: [usize; 3],
    }

    let ans = colors.iter().unique().count();
    println!("{}", ans);
}
