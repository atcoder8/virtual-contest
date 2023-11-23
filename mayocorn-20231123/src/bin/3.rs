use itertools::Itertools;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> i64 {
    input! {
        (n, m): (usize, usize),
        mut xx: [i64; m],
    }

    if n >= m {
        return 0;
    }

    xx.sort_unstable();

    xx.windows(2)
        .map(|window| window[1] - window[0])
        .sorted_unstable()
        .take(m - n)
        .sum()
}
