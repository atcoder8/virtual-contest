use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut hh: [usize; n],
        mut ww: [usize; m],
    }

    hh.sort_unstable();
    ww.sort_unstable();

    let mut prefix_sum_0 = hh
        .chunks_exact(2)
        .map(|chunk| chunk[1] - chunk[0])
        .collect_vec();
    prefix_sum_0.insert(0, 0);
    for i in 0..n / 2 {
        prefix_sum_0[i + 1] += prefix_sum_0[i];
    }

    let mut prefix_sum_1 = hh[1..]
        .chunks_exact(2)
        .map(|chunk| chunk[1] - chunk[0])
        .collect_vec();
    prefix_sum_1.insert(0, 0);
    for i in 0..n / 2 {
        prefix_sum_1[i + 1] += prefix_sum_1[i];
    }

    let calc_min_sum_diff = |pivot: usize| {
        let h = hh[pivot];

        let pos = ww.lower_bound(&h);
        let min_diff = h
            .abs_diff(ww[pos.min(m - 1)])
            .min(h.abs_diff(ww[pos.saturating_sub(1)]));

        prefix_sum_0[pivot / 2] + prefix_sum_1[n / 2] - prefix_sum_1[(pivot + 1) / 2] + min_diff
    };

    let ans = (0..n).step_by(2).map(calc_min_sum_diff).min().unwrap();
    println!("{}", ans);
}
