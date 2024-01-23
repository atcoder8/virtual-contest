use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }

    // 現在ベルトコンベアに入っている商品のうち最もt+dが小さいものを優先する

    let mut ans = 0;
    let mut queue: VecDeque<(usize, usize)> = td.iter().cloned().sorted_by_key(|x| x.0).collect();
    let mut heap = BinaryHeap::<Reverse<usize>>::new();
    let mut cur_time = 0;
    while !queue.is_empty() || !heap.is_empty() {
        if heap.is_empty() {
            cur_time = cur_time.max(queue.front().unwrap().0);
        }

        while let Some(&(t, d)) = queue.front() {
            if t > cur_time {
                break;
            }

            heap.push(Reverse(t + d));
            queue.pop_front();
        }

        if let Some(Reverse(out_time)) = heap.pop() {
            if out_time >= cur_time {
                ans += 1;
                cur_time += 1;
            }
        }
    }

    println!("{}", ans);
}
