use std::collections::BTreeSet;

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }

    xy.sort_unstable_by_key(|&(x, _y)| x);

    let is_ok = |dist: usize| {
        let mut min1 = 10_usize.pow(9);
        let mut max1 = 0;
        let mut pool = BTreeSet::<(usize, usize)>::new();
        for (i, &(_, y)) in enumerate(&xy) {
            pool.insert((y, i));
        }
        let mut right = 0;
        for left in 0..n {
            min1 = min1.min(xy[left].1);
            max1 = max1.max(xy[left].1);

            while right < n && xy[right].0 - xy[left].0 < dist {
                pool.remove(&(xy[right].1, right));
                right += 1;
            }

            if right == n {
                break;
            }

            let min2 = pool.iter().next().unwrap().0;
            let max2 = pool.iter().next_back().unwrap().0;

            if max1.saturating_sub(min2) >= dist || max2.saturating_sub(min1) >= dist {
                return true;
            }
        }

        false
    };

    let mut ok = 0_usize;
    let mut ng = 10_usize.pow(9) + 1;
    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
