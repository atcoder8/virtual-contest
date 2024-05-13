use proconio::input;

fn main() {
    input! {
        mut h: usize,
    }

    let mut ans = 0_usize;
    let mut num = 1_usize;

    while h != 0 {
        ans += num;
        h /= 2;
        num *= 2;
    }

    println!("{}", ans);
}
