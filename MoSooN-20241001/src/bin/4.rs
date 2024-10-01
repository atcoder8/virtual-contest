use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut top_k = vec![];
    let mut heap = BinaryHeap::from([Reverse(0_usize)]);
    while top_k.len() < k + 1 {
        let price = heap.pop().unwrap().0;

        if top_k.last().is_some_and(|&last| last == price) {
            continue;
        }

        top_k.push(price);

        heap.extend(aa.iter().map(|&a| Reverse(price + a)));
    }

    println!("{}", top_k[k]);
}
