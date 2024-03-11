use num::Integer;
use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", a.lcm(&b));
}
