use std::cmp::Reverse;

use itertools::{izip, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [usize; 3],
        aa: [usize; n],
        bb: [usize; n],
    }

    let evaluation_formulas = [
        |a: usize, _b: usize| a,
        |_a: usize, b: usize| b,
        |a: usize, b: usize| a + b,
    ];

    let mut accepted: Vec<usize> = vec![];
    let mut examinees = (0..n).collect_vec();
    for (formula, accept_num) in izip!(evaluation_formulas, xyz) {
        examinees.sort_unstable_by_key(|&i| (Reverse(formula(aa[i], bb[i])), i));
        accepted.extend(&examinees[..accept_num]);
        examinees = examinees[accept_num..].to_owned();
    }
    accepted.sort_unstable();

    println!("{}", accepted.iter().map(|i| i + 1).join("\n"));
}
