use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        xx: [Usize1; q],
    }

    let mut idx_to_val = (0..n).collect_vec();
    let mut val_to_idx = (0..n).collect_vec();
    for &x in &xx {
        let i = val_to_idx[x];
        let j = if i < n - 1 { i + 1 } else { i - 1 };
        let y = idx_to_val[j];

        idx_to_val.swap(i, j);
        val_to_idx.swap(x, y);
    }

    println!("{}", idx_to_val.iter().map(|a| a + 1).join(" "));
}
