use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut queue = VecDeque::from([n]);
    for i in (0..n).rev() {
        if s[i] == 'L' {
            queue.push_back(i);
        } else {
            queue.push_front(i)
        }
    }

    println!("{}", queue.iter().join(" "));
}
