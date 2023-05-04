use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; m],
    }

    let mut bb = (0..n).collect_vec();
    let mut others = vec![None; m];

    for (i, &a) in aa.iter().enumerate() {
        if bb[a] == 0 {
            others[i] = Some(bb[a + 1]);
        } else if bb[a + 1] == 0 {
            others[i] = Some(bb[a]);
        }

        bb.swap(a, a + 1);
    }

    let zero_idx = bb.iter().find_position(|&&b| b == 0).unwrap().0;

    let mut inv_bb = vec![0; n];
    for (i, &b) in bb.iter().enumerate() {
        inv_bb[b] = i;
    }

    let mut ss = vec![0; m];
    for i in 0..m {
        if let Some(other) = others[i] {
            ss[i] = inv_bb[other];
        } else {
            ss[i] = zero_idx;
        }
    }

    for &s in &ss {
        println!("{}", s + 1);
    }
}
