use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [f64; n],
    }

    let ans = 1.0 / aa.iter().map(|a| 1.0 / a).sum::<f64>();
    println!("{}", ans);
}
