use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> i64 {
    input! {
        (n, _m): (usize, usize),
        mut xy: [(i64, i64); n],
    }

    let c0 = xy[0].0;
    xy[0].1 -= 1;

    let mut ans = c0;
    let mut diff = c0;
    let mut sum = c0;
    for &(x, y) in &xy {
        let term = if x >= 0 { y } else { (diff.max(0) / -x).min(y) };

        ans = ans.max(sum + diff * term + x * term * (term + 1) / 2);
        sum += diff * y + x * y * (y + 1) / 2;
        diff += x * y;
    }

    ans
}
