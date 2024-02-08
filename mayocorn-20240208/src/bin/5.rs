use std::collections::BTreeSet;

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        aa: [usize; n],
    }

    let prefix = enumerate(&aa[..m])
        .map(|(i, &a)| (a, i))
        .sorted_unstable()
        .collect_vec();
    let mut small: BTreeSet<(usize, usize)> = prefix[..k].iter().cloned().collect();
    let mut large: BTreeSet<(usize, usize)> = prefix[k..].iter().cloned().collect();

    let mut sum = small.iter().map(|&(a, _)| a).sum::<usize>();

    let mut ans = vec![sum];
    for i in m..n {
        let rm = (aa[i - m], i - m);
        if small.contains(&rm) {
            small.remove(&rm);
            sum -= rm.0;

            if !large.is_empty() {
                let mv = large.pop_first().unwrap();
                small.insert(mv);
                sum += mv.0;
            }
        } else {
            large.remove(&rm);
        }

        let add = (aa[i], i);
        small.insert(add);
        sum += add.0;

        if small.len() > k {
            let mv = small.pop_last().unwrap();
            sum -= mv.0;
            large.insert(mv);
        }

        ans.push(sum);
    }

    println!("{}", ans.iter().join(" "));
}
