use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    println!("{}", (b / a).min(c));
}
