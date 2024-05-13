use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans + 1),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut strongest = vec![true; n];
    for &(_, b) in &ab {
        strongest[b] = false;
    }

    let positions = strongest.iter().positions(|&x| x).collect_vec();

    if positions.len() != 1 {
        return None;
    }

    Some(positions[0])
}
