use proconio::{input, marker::Usize1};

fn main() {
    let ans = match solve() {
        Some(ans) => format!("{}", ans),
        None => format!("-1"),
    };
    println!("{}", ans);
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut cur = 0;
    for cnt in 0..n {
        if cur == 1 {
            return Some(cnt);
        }

        cur = aa[cur];
    }

    None
}
