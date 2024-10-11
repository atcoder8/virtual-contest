use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let ans = (n - 1) * (m - 1);
    println!("{}", ans);
}
