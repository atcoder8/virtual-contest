use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    let mut ans = 0;
    let mut rem = m;
    for &(a, b) in ab.iter().sorted_unstable() {
        if b >= rem {
            ans += a * rem;
            break;
        }

        ans += a * b;
        rem -= b;
    }

    println!("{}", ans);
}
