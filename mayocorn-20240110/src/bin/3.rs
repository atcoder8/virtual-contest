use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        pp: [usize; n],
    }

    let mut heap = BinaryHeap::from(pp[..k].iter().map(|&p| Reverse(p)).collect_vec());

    println!("{}", heap.peek().unwrap().0);
    for &p in &pp[k..] {
        heap.push(Reverse(p));
        heap.pop();

        println!("{}", heap.peek().unwrap().0);
    }
}
