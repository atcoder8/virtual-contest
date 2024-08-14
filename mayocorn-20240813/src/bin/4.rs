use proconio::input;

const MAX: usize = 2 * 10_usize.pow(5);

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut counts = vec![0_usize; MAX + 1];
    for &a in &aa {
        counts[a] += 1;
    }

    let mut ans = 0_usize;
    for denom in 1..=MAX {
        for numer in (denom..=MAX).step_by(denom) {
            ans += counts[numer] * counts[denom] * counts[numer / denom];
        }
    }
    println!("{}", ans);
}
