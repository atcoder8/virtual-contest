use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }

    xy.sort_unstable_by_key(|k| k.0);

    let is_ok = |low_lim: usize| {
        let mut left = 0;
        let mut min_y = 10_usize.pow(9);
        let mut max_y = 0;

        for right in 0..n {
            let (right_x, right_y) = xy[right];

            while left < right {
                let (left_x, left_y) = xy[left];

                if right_x - left_x < low_lim {
                    break;
                }

                min_y = min_y.min(left_y);
                max_y = max_y.max(left_y);

                left += 1;
            }

            if right_y.saturating_sub(min_y) >= low_lim || max_y.saturating_sub(right_y) >= low_lim
            {
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
