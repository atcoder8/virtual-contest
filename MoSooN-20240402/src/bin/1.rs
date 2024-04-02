use itertools::{izip, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, p, q, r, s): (usize, Usize1, usize, Usize1, usize),
        mut aa: [usize; n],
    }

    for (i, j) in izip!(p..q, r..s) {
        aa.swap(i, j);
    }

    println!("{}", aa.iter().join(" "));
}
