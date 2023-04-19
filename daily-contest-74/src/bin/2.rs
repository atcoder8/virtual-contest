use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q, s, t): (usize, usize, i64, i64),
        aa: [i64; n + 1],
        lrx: [(Usize1, usize, i64); q],
    }

    let temperature_change = |diff: i64| {
        if diff > 0 {
            -s * diff
        } else {
            -t * diff
        }
    };

    let mut diffs = aa.windows(2).map(|w| w[1] - w[0]).collect_vec();

    let mut temperature: i64 = diffs.iter().map(|&diff| temperature_change(diff)).sum();

    for &(l, r, x) in &lrx {
        temperature += temperature_change(diffs[l] + x) - temperature_change(diffs[l]);
        if r < n {
            temperature += temperature_change(diffs[r] - x) - temperature_change(diffs[r]);
        }

        diffs[l] += x;
        if r < n {
            diffs[r] -= x;
        }

        println!("{}", temperature);
    }
}
