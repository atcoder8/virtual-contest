// unfinished

use std::{cmp::Reverse, collections::BTreeSet};

use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m, l): (usize, usize, usize),
        aa: [usize; n],
        bb: [usize; m],
        cd: [(Usize1, Usize1); l],
    }

    let bad: BTreeSet<(usize, usize)> = cd.into_iter().collect();

    let ia = enumerate(aa)
        .sorted_unstable_by_key(|v| Reverse(v.1))
        .collect_vec();
    let ib = enumerate(bb)
        .sorted_unstable_by_key(|v| Reverse(v.1))
        .collect_vec();

    let mut start_pos = 0;
    let mut progresses = vec![0; n];

    loop {
        for pos in start_pos..n {
            let (a_idx, a) = ia[pos];

            if pos < n - 1
                && a + ib[progresses[pos]].1 < ia[start_pos].1 + ib[progresses[start_pos]].1
            {
                break;
            }

            while progresses[pos] < m {
                let (b_idx, b) = ib[progresses[pos]];
                let score = a + b;
                if pos < n - 1 && score < ia[pos + 1].1 + ib[progresses[pos + 1]].1 {
                    break;
                }

                if !bad.contains(&(a_idx, b_idx)) {
                    return score;
                }

                progresses[pos] += 1;
            }

            if progresses[pos] == m {
                start_pos += 1;
            }
        }
    }
}
