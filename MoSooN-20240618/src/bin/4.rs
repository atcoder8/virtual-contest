use std::iter::zip;

use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        s: String,
    }

    let mut prefix_m = [0; 3];
    let mut suffix_x = [0; 3];
    for (&a, c) in zip(&aa, s.chars()) {
        suffix_x[a] += (c == 'X') as usize;
    }

    let mut ans = 0;
    for (&a, c) in zip(&aa, s.chars()) {
        match c {
            'M' => prefix_m[a] += 1,
            'E' => {
                for (i, j) in iproduct!(0..3, 0..3) {
                    let mex = (0..=3).find(|cand| ![i, a, j].contains(cand)).unwrap();
                    ans += prefix_m[i] * suffix_x[j] * mex;
                }
            }
            'X' => suffix_x[a] -= 1,
            _ => panic!(),
        }
    }

    println!("{}", ans);
}
