use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!(
        "{}",
        (0..t)
            .map(|_| if solve() { "Yes" } else { "No" })
            .join("\n")
    );
}

fn solve() -> bool {
    input! {
        (a, s): (usize, usize),
    }

    let mut sum = (0..60)
        .filter(|&d| a >> d & 1 == 1)
        .map(|d| 2 * 2_usize.pow(d))
        .sum::<usize>();
    for d in (0..60).rev() {
        if a >> d & 1 == 1 {
            continue;
        }

        if sum + 2_usize.pow(d) <= s {
            sum += 2_usize.pow(d);
        }
    }

    sum == s
}
