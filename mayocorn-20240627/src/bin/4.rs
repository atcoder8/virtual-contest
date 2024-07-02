use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut heap: BinaryHeap<(usize, usize)> = aa
        .iter()
        .sorted_unstable()
        .dedup_with_count()
        .map(|(cnt, &a)| (a, cnt))
        .chain([(0, k)])
        .collect();
    let mut rem = k;

    let mut ans = 0;

    while rem != 0 && heap.len() >= 2 {
        let (a1, cnt1) = heap.pop().unwrap();
        let (a2, cnt2) = heap.pop().unwrap();
        let diff = a1 - a2;

        let consume = rem.min(diff * cnt1);
        rem -= consume;

        let (q, r) = (consume / cnt1, consume % cnt1);
        ans += calc_sum(a1 - q + 1, a1 + 1) * cnt1 + r * (a1 - q);

        if rem == 0 {
            break;
        }

        heap.push((a2, cnt1 + cnt2));
    }

    println!("{}", ans);
}

fn calc_sum(l: usize, r: usize) -> usize {
    (r - l) * (r + l - 1) / 2
}
