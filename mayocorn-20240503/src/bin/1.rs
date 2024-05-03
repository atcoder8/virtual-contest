use proconio::input;

fn main() {
    input! {
        (x, y, z): (usize, usize, usize),
    }

    println!("{}", (x - z) / (y + z));
}
