use proconio::{fastout, input};

fn main() {
    solve();
}

#[fastout]
fn solve() {
    input! {
        (n, m): (usize, i64),
    }

    if n == 1 && m == 0 {
        println!("1 2");
        return;
    }

    if m < 0 || m > n as i64 - 2 {
        println!("-1");
        return;
    }

    let m = m as usize;

    println!("1 {}", 3 + 3 * m);
    for i in 0..(n - 1) {
        println!("{} {}", 2 + 3 * i, 4 + 3 * i);
    }
}
