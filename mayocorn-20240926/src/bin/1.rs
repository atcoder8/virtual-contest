use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s.chars().last().unwrap();
    println!("{}", ans);
}
