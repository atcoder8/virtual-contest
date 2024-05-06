use itertools::enumerate;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m, p): (usize, usize, usize),
        aa: [usize; n],
        mut bb: [usize; m],
    }

    bb.sort_unstable();
    let mut prefix_sum = vec![0; m + 1];
    for (i, &b) in enumerate(&bb) {
        prefix_sum[i + 1] = prefix_sum[i] + b;
    }

    let mut sum = 0;
    for &a in &aa {
        let bound = bb.lower_bound(&p.saturating_sub(a));
        sum += bound * a + prefix_sum[bound] + (m - bound) * p;
    }

    println!("{}", sum);
}
