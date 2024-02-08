use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a.saturating_sub(2 * b);
    println!("{}", ans);
}
