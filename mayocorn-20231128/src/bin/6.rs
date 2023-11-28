use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        ab: [(f64, f64); n],
        cd: [(f64, f64); m],
    }

    let calc_potential = |sugar: f64, water: f64, slope: f64| sugar - (sugar + water) * slope;

    let is_ok = |dest: f64| {
        let potentials = cd
            .iter()
            .map(|&(c, d)| calc_potential(c, d, dest))
            .sorted_unstable_by(|x, y| x.partial_cmp(&y).unwrap())
            .collect_vec();

        let mut cnt = 0;
        for &(a, b) in &ab {
            let potential = calc_potential(a, b, dest);
            cnt += m - potentials.lower_bound_by(|x| x.partial_cmp(&(-potential)).unwrap());
        }

        cnt >= k
    };

    let mut ok = 0.0_f64;
    let mut ng = 1.0;
    while (ok - ng).abs() > 1e-12 {
        let mid = (ok + ng) / 2.0;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", 100.0 * ok);
}
