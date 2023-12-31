use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let ans = n * n.saturating_sub(1) / 2 + m * m.saturating_sub(1) / 2;
    println!("{}", ans);
}
