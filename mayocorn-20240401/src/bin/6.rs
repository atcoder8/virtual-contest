use itertools::Itertools;
use ordered_float::NotNan;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        ab: [(f64, f64); n],
        cd: [(f64, f64); m],
    }

    let is_ok = |key: f64| {
        let potentials = cd
            .iter()
            .map(|&(c, d)| NotNan::new(c - (c + d) * key).unwrap())
            .sorted_unstable()
            .collect_vec();

        let mut cnt = 0;
        for &(a, b) in &ab {
            cnt += m - potentials.lower_bound(&NotNan::new((a + b) * key - a).unwrap());
        }

        cnt >= k
    };

    let mut ok = 0.0;
    let mut ng = 1.1;
    while ng - ok > 1e-12 {
        let mid = (ok + ng) / 2.0;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", 100.0 * ok);
}
