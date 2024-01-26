use std::collections::BinaryHeap;

use itertools::{chain, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, q): (usize, usize, usize),
        mut wv: [(usize, usize); n],
        xx: [usize; m],
        queries: [(Usize1, usize); q],
    }

    wv.sort_unstable_by_key(|x| x.0);

    let mut ans = vec![];
    ans.reserve(n);
    for &(l, r) in &queries {
        let rem_boxes = chain(&xx[..l], &xx[r..])
            .cloned()
            .sorted_unstable()
            .collect_vec();
        let mut heap = BinaryHeap::<usize>::new();
        let mut idx = 0;
        let mut sum_value = 0;
        for box_size in rem_boxes {
            while idx < n && wv[idx].0 <= box_size {
                heap.push(wv[idx].1);
                idx += 1;
            }

            if let Some(value) = heap.pop() {
                sum_value += value;
            }
        }

        ans.push(sum_value);
    }

    println!("{}", ans.iter().join("\n"));
}
