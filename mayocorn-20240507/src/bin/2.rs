use proconio::input;

const LIMIT: usize = 10_usize.pow(18);

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        aa: [usize; n],
    }

    if aa.contains(&0) {
        return Some(0);
    }

    let mut ans = 1;
    for &a in &aa {
        if a > LIMIT / ans {
            return None;
        }

        ans *= a;
    }

    Some(ans)
}
