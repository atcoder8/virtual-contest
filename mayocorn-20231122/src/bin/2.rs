use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut parents = vec![0; 2 * n + 1];
    for (i, &a) in enumerate(&aa) {
        parents[2 * i + 1] = a;
        parents[2 * i + 2] = a;
    }

    let mut dists = vec![None; 2 * n + 1];
    dists[0] = Some(0);

    for i in 0..2 * n + 1 {
        println!("{}", rec(&parents, &mut dists, i));
    }
}

fn rec(parents: &[usize], dists: &mut Vec<Option<usize>>, cur: usize) -> usize {
    if let Some(dist) = dists[cur] {
        return dist;
    }

    let dist = rec(&parents, dists, parents[cur]) + 1;
    dists[cur] = Some(dist);

    dist
}
