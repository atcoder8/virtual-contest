use proconio::input;

fn main() {
    input! {
        n: usize,
        xu: [(f64, String); n],
    }

    let ans = xu
        .iter()
        .map(|(x, u)| if u == "JPY" { *x } else { 380000.0 * x })
        .sum::<f64>();
    println!("{}", ans);
}
