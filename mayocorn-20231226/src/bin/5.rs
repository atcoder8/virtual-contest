use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![false; n]; n];
    for &(a, b) in &ab {
        graph[a][b] = true;
        graph[b][a] = true;
    }

    let is_ok = |perm: &[usize]| -> bool {
        [0].iter()
            .chain(perm)
            .tuple_windows()
            .all(|(&node1, &node2)| graph[node1][node2])
    };

    let ans = (1..n)
        .permutations(n - 1)
        .filter(|perm| is_ok(perm))
        .count();
    println!("{}", ans);
}
