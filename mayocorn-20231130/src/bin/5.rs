use std::collections::BTreeMap;

use itertools::{izip, Itertools};
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; n],
        cc: [usize; m],
        dd: [usize; m],
    }

    let chocolates = izip!(aa, bb)
        .map(|(a, b)| (a, b))
        .sorted_unstable_by_key(|x| x.0)
        .collect_vec();
    let mut boxes = izip!(cc, dd)
        .map(|(c, d)| (c, d))
        .sorted_unstable_by_key(|x| x.0)
        .collect_vec();

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for (a, b) in chocolates.into_iter().rev() {
        while let Some(&(c, d)) = boxes.last() {
            if c < a {
                break;
            }

            *map.entry(d).or_default() += 1;
            boxes.pop();
        }

        if let Some((&d, cnt)) = map.range_mut(b..).next() {
            *cnt -= 1;

            if *cnt == 0 {
                map.remove(&d);
            }
        } else {
            return false;
        }
    }

    true
}
