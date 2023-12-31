use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut ans = 0;
    let mut cur = 100;
    while cur < x {
        ans += 1;
        cur += cur / 100;
    }
    println!("{}", ans);
}
