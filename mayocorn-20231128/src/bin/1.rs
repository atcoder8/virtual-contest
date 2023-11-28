use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        ll: [usize; n],
    }

    let mut ans = 1;
    let mut cur = 0;
    for &l in &ll {
        cur += l;

        if cur > x {
            break;
        }

        ans += 1;
    }

    println!("{}", ans);
}
