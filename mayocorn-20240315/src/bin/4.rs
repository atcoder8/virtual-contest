use std::collections::BTreeMap;

use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        pp: [Usize1; n],
    }

    let mut turns = vec![None; n];

    let mut stacks = BTreeMap::<usize, Vec<usize>>::new();
    for (turn, &p) in enumerate(&pp) {
        let Some((&key, _)) = stacks.range(p..).next() else {
            if k == 1 {
                turns[p] = Some(turn + 1);
            } else {
                stacks.insert(p, vec![p]);
            }

            continue;
        };

        let mut stack = stacks.remove(&key).unwrap();
        stack.push(p);

        if stack.len() == k {
            for val in stack {
                turns[val] = Some(turn + 1);
            }
        } else {
            stacks.insert(p, stack);
        }
    }

    for &turn in &turns {
        match turn {
            Some(turn) => println!("{}", turn),
            None => println!("-1"),
        }
    }
}
