use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [usize; n],
        tt: [usize; n],
    }

    let mut times = vec![10_usize.pow(10); n];
    let mut heap = BinaryHeap::from(enumerate(&tt).map(|(i, &t)| (Reverse(t), i)).collect_vec());
    while let Some((Reverse(time), cur)) = heap.pop() {
        if times[cur] <= time {
            continue;
        }

        times[cur] = time;

        heap.push((Reverse(time + ss[cur]), (cur + 1) % n));
    }

    println!("{}", times.iter().join("\n"));
}
