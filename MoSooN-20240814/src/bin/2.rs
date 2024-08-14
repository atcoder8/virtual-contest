use proconio::input;

fn main() {
    input! {
        (a, d): (usize, usize),
    }

    let ans = ((a + 1) * d).max(a * (d + 1));
    println!("{}", ans);
}
