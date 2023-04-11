use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let mut ans = 0;
    for b in (k + 1)..=n {
        ans += n / b * (b - k) + (n % b).saturating_sub(k.saturating_sub(1));
    }

    println!("{}", ans);
}
