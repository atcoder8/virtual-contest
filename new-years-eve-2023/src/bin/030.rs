use proconio::input;

fn main() {
    input! {
        (r, d, mut x): (usize, usize, usize),
    }

    for _ in 1..=10 {
        x = r * x - d;
        println!("{}", x);
    }
}
