use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let ans = if k == 1 { 0 } else { n - k };
    println!("{}", ans);
}
