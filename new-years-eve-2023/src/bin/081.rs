use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    let mut max = 0;
    let mut cnt = 0;
    for (&h1, &h2) in hh.iter().tuple_windows() {
        if h1 >= h2 {
            cnt += 1;
            max = max.max(cnt);
        } else {
            cnt = 0;
        }
    }

    println!("{}", max);
}
