use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let ans = a.min(b) + c.min(d);
    println!("{}", ans);
}
