use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (_n, m): (usize, usize),
        lr: [(Usize1, usize); m],
    }

    let (mut left, mut right) = lr[0];
    for &(l, r) in &lr[1..] {
        left = left.max(l);
        right = right.min(r);
    }

    let ans = right.saturating_sub(left);
    println!("{}", ans);
}
