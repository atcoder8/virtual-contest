use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let ans = a.abs_diff(c) <= d || a.abs_diff(b) <= d && b.abs_diff(c) <= d;
    println!("{}", if ans { "Yes" } else { "No" });
}
