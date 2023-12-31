use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let ans = x / 10 + x % 10;
    println!("{}", ans);
}
