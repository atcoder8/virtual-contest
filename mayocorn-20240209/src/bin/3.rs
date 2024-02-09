use proconio::input;

fn main() {
    input! {
        (n, k): (u32, usize),
    }

    let ans = k * (k - 1).pow(n - 1);
    println!("{}", ans);
}
