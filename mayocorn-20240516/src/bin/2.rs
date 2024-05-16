use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [Usize1; k],
        xy: [(f64, f64); n],
    }

    let is_ok = |strength: f64| {
        let mut illuminated = vec![false; n];
        for &a in &aa {
            let source = xy[a];

            for (i, &other) in enumerate(&xy) {
                if calc_dist(source, other) <= strength {
                    illuminated[i] = true;
                }
            }
        }

        illuminated.iter().all(|&v| v)
    };

    let mut ok = 1e6;
    let mut ng = 0.0_f64;

    while (ok - ng).abs() > 1e-7 {
        let mid = (ok + ng) / 2.0;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn calc_dist(coord1: (f64, f64), coord2: (f64, f64)) -> f64 {
    (coord1.0 - coord2.0).hypot(coord1.1 - coord2.1)
}
