use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by_key(|v| v.1);

    let mut cur = 0;
    for &(a, b) in &ab {
        cur += a;

        if cur > b {
            return false;
        }
    }

    true
}
