use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
    }

    let unit = 2 * d + 1;
    let ans = (n + unit - 1) / unit;
    println!("{}", ans);
}
