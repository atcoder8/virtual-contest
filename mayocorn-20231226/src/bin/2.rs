use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..=5 {
        ans += n.saturating_sub(1000_usize.pow(i) - 1);
    }
    println!("{}", ans);
}
