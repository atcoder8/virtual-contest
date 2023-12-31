use proconio::input;

fn main() {
    input! {
        mut h: usize,
    }

    let mut ans = 0_usize;
    let mut cnt = 1_usize;
    while h != 0 {
        ans += cnt;
        cnt *= 2;
        h /= 2;
    }

    println!("{}", ans);
}
