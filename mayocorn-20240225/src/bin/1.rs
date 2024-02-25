use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
        k: usize,
    }

    let max = a.max(b).max(c);
    let ans = a + b + c - max + (max << k);
    println!("{}", ans);
}
