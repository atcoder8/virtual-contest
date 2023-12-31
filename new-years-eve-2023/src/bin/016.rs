use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0;
    let mut cur = 1;
    for &a in &aa {
        if a == cur {
            cur += 1;
        } else {
            ans += 1;
        }
    }

    if ans < n {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
