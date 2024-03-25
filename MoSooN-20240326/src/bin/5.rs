use itertools::izip;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, k): (usize, usize),
        mut aa: [usize; n],
        mut ff: [usize; n],
    }

    aa.sort_unstable();
    ff.sort_unstable();
    ff.reverse();

    let is_ok = |cost_limit: usize| {
        let mut train_num = 0;
        for (&a, &f) in izip!(&aa, &ff) {
            let a_lim = cost_limit / f;
            train_num += a.saturating_sub(a_lim);
        }

        train_num <= k
    };

    if is_ok(0) {
        return 0;
    }

    let mut ok = aa[n - 1] * ff[0];
    let mut ng = 0_usize;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}
