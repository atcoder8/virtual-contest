use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        mut vv: [usize; n],
    }

    vv.sort_unstable();

    let top_vv = &vv[n - a..];
    let max_average = top_vv.iter().sum::<usize>() as f64 / a as f64;

    let top_counter = top_vv.iter().rev().dedup_with_count().collect_vec();
    let (last_cnt, &last_v) = *top_counter.last().unwrap();
    let lower_bound = vv.lower_bound(&last_v);
    let upper_bound = vv.upper_bound(&last_v);
    let num_last_v = upper_bound - lower_bound;

    let max_cnt = if top_counter.len() == 1 {
        b.min(num_last_v)
    } else {
        last_cnt
    };

    let num_combs = (last_cnt..=max_cnt)
        .map(|cnt| calc_combinations(num_last_v, cnt))
        .sum::<usize>();

    println!("{}\n{}", max_average, num_combs);
}

/// Calculates the number of combinations that select `k` elements from `n` elements.
pub fn calc_combinations(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    let k = k.min(n - k);
    let mut ret = 1;
    for i in 0..k {
        ret *= n - i;
        ret /= i + 1;
    }

    ret
}
