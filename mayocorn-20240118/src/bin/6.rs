use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        mut td: [(Usize1, usize); n],
    }

    struct Sushi {
        topping: usize,
        delicious: usize,
    }

    let sushi_vec = td
        .iter()
        .map(|&(t, d)| Sushi {
            topping: t,
            delicious: d,
        })
        .sorted_by_key(|sushi| Reverse(sushi.delicious))
        .collect_vec();

    let (selected_sushi, rem_sushi) = sushi_vec.split_at(k);

    let mut scores_per_type = vec![vec![]; n];
    for sushi in selected_sushi {
        scores_per_type[sushi.topping].push(sushi.delicious);
    }
    scores_per_type
        .iter_mut()
        .for_each(|scores| scores.sort_unstable_by_key(|&score| Reverse(score)));

    let mut replaceable_heap = BinaryHeap::new();
    for scores in &scores_per_type {
        replaceable_heap.extend(scores.iter().skip(1).map(|&score| Reverse(score)));
    }

    let mut rem_heap: BinaryHeap<(usize, usize)> = rem_sushi
        .iter()
        .map(|sushi| (sushi.delicious, sushi.topping))
        .collect();

    let mut used = vec![false; n];
    for sushi in selected_sushi {
        used[sushi.topping] = true;
    }

    let mut base_delicious = selected_sushi
        .iter()
        .map(|sushi| sushi.delicious)
        .sum::<usize>();
    let mut topping_type_num = scores_per_type
        .iter()
        .filter(|scores| !scores.is_empty())
        .count();
    let mut ans = base_delicious + topping_type_num.pow(2);

    let select_add_sushi = |rem_heap: &mut BinaryHeap<(usize, usize)>, used: &mut [bool]| {
        while let Some((delicious, topping)) = rem_heap.pop() {
            if !used[topping] {
                used[topping] = true;

                return Some(Sushi { delicious, topping });
            }
        }

        None
    };

    while let Some(Reverse(remove_delicious)) = replaceable_heap.pop() {
        base_delicious -= remove_delicious;

        let Some(add_sushi) = select_add_sushi(&mut rem_heap, &mut used) else { break; };
        base_delicious += add_sushi.delicious;
        topping_type_num += 1;

        let sum_score = base_delicious + topping_type_num.pow(2);
        ans = ans.max(sum_score);
    }

    println!("{}", ans);
}
