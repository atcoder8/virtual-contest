use itertools::Itertools;
use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aaa: [[Usize1; n]; m],
    }

    let mut mat = Array2::from_elem((n, n), true);
    for aa in &aaa {
        for (&a1, &a2) in aa.iter().tuple_windows() {
            mat[(a1, a2)] = false;
            mat[(a2, a1)] = false;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans += mat[(i, j)] as usize;
        }
    }
    println!("{}", ans);
}
