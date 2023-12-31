use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        y: usize,
    }

    let ans = x * n.min(k) + y * n.saturating_sub(k);
    println!("{}", ans);
}
