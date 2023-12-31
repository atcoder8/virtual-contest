use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        (w, h): (usize, usize),
    }

    println!("{}", if w / w.gcd(&h) == 4 { "4:3" } else { "16:9" });
}
