use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [[Usize1]; m],
    }

    let bits = ss
        .iter()
        .map(|s| s.iter().map(|&a| 1_usize << a).sum::<usize>())
        .collect_vec();

    let is_ok = |select: usize| {
        let mut union = 0;
        for i in 0..m {
            if select >> i & 1 == 1 {
                union |= bits[i];
            }
        }

        union == (1 << n) - 1
    };

    let ans = (0..1 << m).filter(|&select| is_ok(select)).count();
    println!("{}", ans);
}
