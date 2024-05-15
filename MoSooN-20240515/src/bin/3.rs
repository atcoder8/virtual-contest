use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (_n, m, x): (usize, usize, usize),
        aa: [usize; m],
    }

    let pos = aa.upper_bound(&x);
    let ans = pos.min(m - pos);
    println!("{}", ans);
}
