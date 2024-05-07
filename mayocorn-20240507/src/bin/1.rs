use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s.chars().position(|c| c == '1').unwrap() % 2 == 0;
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
