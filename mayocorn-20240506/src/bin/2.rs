use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        o: String,
        e: String,
    }

    let password = o.chars().interleave(e.chars()).collect::<String>();
    println!("{}", password);
}
