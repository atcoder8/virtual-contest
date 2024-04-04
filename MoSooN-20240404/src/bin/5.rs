use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [i64; n],
    }

    let mut a = 0;
    let mut b = 0;
    let mut cc = vec![0; n + 2];

    for (i, &s) in enumerate(&ss) {
        cc[i + 2] = s - (cc[i] + cc[i + 1]);

        match i % 3 {
            1 => a = a.max(-cc[i + 2]),
            2 => b = b.max(-cc[i + 2]),
            _ => {}
        }
    }

    let aa = enumerate(&cc)
        .map(|(i, &c)| match i % 3 {
            0 => c + a,
            1 => c + b,
            2 => c - (a + b),
            _ => panic!(),
        })
        .collect_vec();

    if aa.iter().all(|&a| a >= 0) {
        println!("Yes\n{}", aa.iter().join(" "));
    } else {
        println!("No");
    }
}
