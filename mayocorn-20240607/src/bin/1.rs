use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = ((n + 1) / 2) as f64 / n as f64;
    println!("{}", ans);
}
