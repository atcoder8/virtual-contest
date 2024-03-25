use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", if a * b % 2 == 1 { "Odd" } else { "Even" });
}
