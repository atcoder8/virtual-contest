use proconio::input;

fn main() {
    input! {
        l: f64,
    }

    let ans = (l / 3.0).powi(3);
    println!("{}", ans);
}
