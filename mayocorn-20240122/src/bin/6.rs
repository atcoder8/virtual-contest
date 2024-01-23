use std::{cmp::Reverse, collections::BinaryHeap};

use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        abcd: [(Usize1, Usize1, usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, c, d) in &abcd {
        graph[a].push((b, c, d));
        graph[b].push((a, c, d));
    }

    let mut min_times = vec![None; n];
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(time), cur)) = heap.pop() {
        if min_times[cur].is_some() {
            continue;
        }

        min_times[cur] = Some(time);

        for &(next, c, d) in &graph[cur] {
            let calc_next_time = |start_time: usize| start_time + c + (d / (start_time + 1));

            let best_start_time = d.sqrt().saturating_sub(1);

            let next_time = calc_next_time(best_start_time.max(time))
                .min(calc_next_time((best_start_time + 1).max(time)));
            heap.push((Reverse(next_time), next));
        }
    }

    min_times[n - 1]
}
