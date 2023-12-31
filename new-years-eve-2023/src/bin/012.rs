use proconio::input;

fn main() {
    input! {
        (height, bmi): (f64, f64),
    }

    let ans = bmi * (height / 100.0).powi(2);
    println!("{}", ans);
}
