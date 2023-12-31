use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
        bb: [usize; n],
        cc: [usize; n - 1],
    }

    let ans = bb[aa[0]]
        + aa.iter()
            .tuple_windows()
            .map(|(&a1, &a2)| bb[a2] + if a1 + 1 == a2 { cc[a1] } else { 0 })
            .sum::<usize>();
    println!("{}", ans);
}
