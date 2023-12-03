use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0;
    for &a in &aa {
        let mut t = a;
        while t % 2 == 0 {
            t /= 2;
            ans += 1;
        }
    }

    println!("{}", ans);
}
