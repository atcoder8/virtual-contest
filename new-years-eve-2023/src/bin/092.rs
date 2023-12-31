use itertools::izip;
use proconio::input;

fn main() {
    input! {
        (l, r): (usize, usize),
        ll: [usize; l],
        rr: [usize; r],
    }

    let mut left_counts = vec![0; 50];
    for &l in &ll {
        left_counts[l] += 1;
    }
    let mut right_counts = vec![0; 50];
    for &r in &rr {
        right_counts[r] += 1;
    }

    let ans = izip!(left_counts, right_counts)
        .map(|(left_cnt, right_cnt)| left_cnt.min(right_cnt))
        .sum::<usize>();
    println!("{}", ans);
}
