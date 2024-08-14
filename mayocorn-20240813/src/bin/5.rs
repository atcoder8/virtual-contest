use std::collections::VecDeque;

use im_rc::HashSet;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    let init_discs = (0..n + 2)
        .map(|i| if i < n { s[i] } else { '.' })
        .collect_vec();
    let mut queue = VecDeque::from([(init_discs, 0_usize)]);
    let mut used = HashSet::<Vec<char>>::new();
    while let Some((discs, cost)) = queue.pop_front() {
        if discs[..n] == t {
            return Some(cost);
        }

        if used.contains(&discs) {
            continue;
        }

        used.insert(discs.clone());

        let empty_pos = (0..n + 1)
            .find(|&i| discs[i] == '.' && discs[i + 1] == '.')
            .unwrap();

        for x in 0..n + 1 {
            if discs[x] == '.' || discs[x + 1] == '.' {
                continue;
            }

            let mut next_discs = discs.clone();
            next_discs.swap(x, empty_pos);
            next_discs.swap(x + 1, empty_pos + 1);

            queue.push_back((next_discs, cost + 1));
        }
    }

    None
}
