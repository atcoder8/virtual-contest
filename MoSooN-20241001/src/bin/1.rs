use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (i64, i64, i64, i64),
    }

    println!("{}\nTakahashi", (a + b) * (c - d));
}
