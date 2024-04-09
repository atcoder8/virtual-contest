use proconio::input;

fn main() {
    input! {
        (d, n): (u32, usize),
    }

    let base = 100_usize.pow(d);
    let ans = (base..)
        .step_by(base)
        .filter(|&i| i % (100 * base) != 0)
        .nth(n - 1)
        .unwrap();
    println!("{}", ans);
}
