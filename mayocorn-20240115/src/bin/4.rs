use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::izip;
use proconio::input;

fn main() {
    input! {
        (n, a, b, c): (usize, usize, usize, usize),
        ddd: [[usize; n]; n],
    }

    let mut cur_dists = vec![None; n];
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(cand_cost), cur)) = heap.pop() {
        if update_cost(&mut cur_dists[cur], cand_cost) {
            for next in 0..n {
                let cand_next_cost = cand_cost + ddd[cur][next] * a;
                heap.push((Reverse(cand_next_cost), next));
            }
        }
    }

    let mut train_dists = vec![None; n];
    let mut heap = BinaryHeap::from([(Reverse(0), n - 1)]);
    while let Some((Reverse(cand_cost), cur)) = heap.pop() {
        if update_cost(&mut train_dists[cur], cand_cost) {
            for next in 0..n {
                let cand_next_cost = cand_cost + ddd[cur][next] * b + c;
                heap.push((Reverse(cand_next_cost), next));
            }
        }
    }

    let ans = izip!(cur_dists, train_dists)
        .map(|(dist1, dist2)| dist1.unwrap() + dist2.unwrap())
        .min()
        .unwrap();
    println!("{}", ans);
}

/// Updates the minimum cost.
/// If `cost` is `None`, always updated to the candidate cost.
///
/// # Arguments
///
/// * `cost` - Reference variable for the cost to be updated.
/// * `cand_cost` - Candidate cost to update.
pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: PartialOrd,
{
    if cost.as_ref().is_some_and(|cost| cost <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
