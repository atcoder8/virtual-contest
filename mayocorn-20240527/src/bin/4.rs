use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        tws: [(usize, usize, usize); m],
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Event {
        Return(usize),
        Flow(usize, usize),
    }

    let mut scores = vec![0; n];

    let mut heap: BinaryHeap<Reverse<(usize, Event)>> = tws
        .iter()
        .map(|&(t, w, s)| Reverse((t, Event::Flow(w, s))))
        .collect();
    let mut queue: BinaryHeap<Reverse<usize>> = (0..n).map(|i| Reverse(i)).collect();

    while let Some(Reverse((time, event))) = heap.pop() {
        match event {
            Event::Return(person) => queue.push(Reverse(person)),
            Event::Flow(w, s) => {
                let Some(Reverse(person)) = queue.pop() else { continue; };
                scores[person] += w;

                heap.push(Reverse((time + s, Event::Return(person))));
            }
        }
    }

    println!("{}", scores.iter().join("\n"));
}
