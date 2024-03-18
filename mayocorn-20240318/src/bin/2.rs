use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut counts = vec![0_usize; 200];
    for &a in &aa {
        counts[a % 200] += 1;
    }

    let ans = counts
        .iter()
        .map(|&cnt| cnt * cnt.saturating_sub(1) / 2)
        .sum::<usize>();
    println!("{}", ans);
}
