use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        xx: [i64; m],
    }

    let ans = xx
        .iter()
        .sorted_unstable()
        .tuple_windows()
        .map(|(&x1, &x2)| x2 - x1)
        .sorted_unstable()
        .take(m.saturating_sub(n))
        .sum::<i64>();
    println!("{}", ans);
}
