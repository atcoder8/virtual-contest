use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abx: [(usize, usize, Usize1); n - 1],
    }

    let mut min_times = vec![10_usize.pow(15); n];
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(time), cur)) = heap.pop() {
        if time >= min_times[cur] {
            continue;
        }

        min_times[cur] = time;

        if cur == n - 1 {
            break;
        }

        let (a, b, x) = abx[cur];
        heap.push((Reverse(time + a), cur + 1));
        heap.push((Reverse(time + b), x));
    }

    println!("{}", min_times[n - 1]);
}
