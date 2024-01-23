use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", n.sqrt().pow(2));
}
