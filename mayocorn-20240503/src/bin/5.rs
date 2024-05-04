use im_rc::HashSet;
use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
        q: usize,
        xy: [(usize, usize); q],
    }

    let mut a_idx = 0;
    let mut set_a = HashSet::new();
    let mut boundaries_a = vec![0];

    let mut b_idx = 0;
    let mut set_b = HashSet::new();
    let mut boundaries_b = vec![0];

    let mut diff = 0;
    let mut same_each_len = vec![true];

    while a_idx < n || b_idx < n {
        if a_idx < n {
            let a = aa[a_idx];

            if set_b.contains(&a) {
                diff -= 1;
            } else {
                diff += 1;
            }

            set_a.insert(a);

            while a_idx < n && set_a.contains(&aa[a_idx]) {
                a_idx += 1;
            }

            boundaries_a.push(a_idx);
        }

        if b_idx < n {
            let b = bb[b_idx];

            if set_a.contains(&b) {
                diff -= 1;
            } else {
                diff += 1;
            }

            set_b.insert(b);

            while b_idx < n && set_b.contains(&bb[b_idx]) {
                b_idx += 1;
            }

            boundaries_b.push(b_idx);
        }

        same_each_len.push(diff == 0);
    }

    let is_ok = |x: usize, y: usize| {
        let len = boundaries_a.lower_bound(&x);

        if !same_each_len[len] || len >= boundaries_b.len() {
            return false;
        }

        boundaries_b[len - 1] < y && y <= boundaries_b[len]
    };

    let ans = xy
        .iter()
        .map(|&(x, y)| if is_ok(x, y) { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}
