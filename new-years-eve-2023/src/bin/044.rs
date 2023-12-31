use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = a + b >= c;
    println!("{}", if ans { "Yes" } else { "No" });
}
