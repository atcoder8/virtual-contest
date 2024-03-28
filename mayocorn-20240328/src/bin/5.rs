use std::collections::BTreeSet;

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut set = BTreeSet::<(usize, usize)>::new();
    for (i, &a) in enumerate(&aa) {
        if let Some(v) = set.range(..(a, 0)).next_back().cloned() {
            set.remove(&v);
        }

        set.insert((a, i));
    }

    println!("{}", set.len());
}
