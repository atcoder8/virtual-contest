use proconio::input;

fn main() {
    input! {
        (h1, m1): (i64, i64),
        (h2, m2): (i64, i64),
        k: i64,
    }

    let ans = 60 * (h2 - h1) + m2 - m1 - k;
    println!("{}", ans);
}
