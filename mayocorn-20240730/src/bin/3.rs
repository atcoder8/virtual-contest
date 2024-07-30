use num_traits::Inv;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> f64 {
    input! {
        (a, b): (f64, f64),
    }

    let calc_time = |inc: usize| a / (1.0 + inc as f64).sqrt() + b * inc as f64;

    if 1.0 - 2.0_f64.sqrt().inv() < b / a {
        return calc_time(0);
    }

    let mut ok = 0_usize;
    let mut ng = 10_usize.pow(18);

    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if (1.0 + mid as f64).sqrt().inv() - (2.0 + mid as f64).sqrt().inv() > b / a {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    calc_time(ng)
}
