use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for d in 1..=n {
        for k in (d..=n).step_by(d) {
            ans += k;
        }
    }

    println!("{}", ans);
}
