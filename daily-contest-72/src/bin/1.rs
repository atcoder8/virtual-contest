use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let mut heap: BinaryHeap<usize> = aa.iter().cloned().collect();
    for _ in 0..m {
        let max = heap.pop().unwrap();
        heap.push(max / 2);
    }

    let ans: usize = heap.iter().sum();
    println!("{}", ans);
}
