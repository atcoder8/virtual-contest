use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        csf: [(usize, usize, usize); n - 1],
    }

    let calc_time = |start: usize| {
        let mut time = 0;
        for &(c, s, f) in &csf[start..] {
            if time <= s {
                time = s + c;
            } else {
                time = (time + f - 1) / f * f + c;
            }
        }

        time
    };

    let ans = (0..n).map(calc_time).join("\n");
    println!("{}", ans);
}
