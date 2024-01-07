use std::cmp::Reverse;

use itertools::{enumerate, izip, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        pass_num: [usize; 3],
        aa: [usize; n],
        bb: [usize; n],
    }

    let mut iab = enumerate(izip!(&aa, &bb))
        .map(|(i, (&a, &b))| (i, a, b))
        .collect_vec();

    let mut accepted = vec![];

    select(&mut accepted, &mut iab, |a, _| a, pass_num[0]);
    select(&mut accepted, &mut iab, |_, b| b, pass_num[1]);
    select(&mut accepted, &mut iab, |a, b| a + b, pass_num[2]);

    accepted.sort_unstable();

    println!("{}", accepted.iter().map(|idx| idx + 1).join("\n"));
}

fn select<F>(accepted: &mut Vec<usize>, iab: &mut Vec<(usize, usize, usize)>, f: F, pass_num: usize)
where
    F: Fn(usize, usize) -> usize,
{
    iab.sort_unstable_by_key(|&(i, a, b)| (Reverse(f(a, b)), i));
    let (ac, rem) = iab.split_at(pass_num);
    accepted.extend(ac.iter().map(|x| x.0));
    *iab = rem.to_owned();
}
