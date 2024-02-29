use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
        k: usize,
    }

    let digits = n
        .iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let mut less = vec![0; k + 1];
    let mut equal = vec![0; k + 1];
    equal[0] = 1;

    let mut ans = 0;

    for &d in &digits {
        let mut next_less = less.clone();

        for cnt in 0..k {
            next_less[cnt + 1] += 9 * less[cnt];
        }

        if d != 0 {
            for cnt in 0..=k {
                next_less[cnt] += equal[cnt];
            }

            for cnt in 0..k {
                next_less[cnt + 1] += (d - 1) * equal[cnt];
            }
        }

        less = next_less;

        if d != 0 {
            equal.pop();
            equal.insert(0, 0);
        }
    }

    ans += less[k] + equal[k];

    println!("{}", ans);
}
