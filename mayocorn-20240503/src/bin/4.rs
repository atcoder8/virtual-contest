use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        csf: [(usize, usize, usize); n - 1],
    }

    let ans = (0..n)
        .map(|start| {
            csf[start..]
                .iter()
                .fold(0, |time, &(c, s, f)| (time.max(s) + f - 1) / f * f + c)
        })
        .join("\n");
    println!("{}", ans);
}
