use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let ans = b.min(d).saturating_sub(a.max(c));
    println!("{}", ans);
}
