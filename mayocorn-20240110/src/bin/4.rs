use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        cc: [Usize1; n],
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut counts = vec![0; 10_usize.pow(5)];
    let mut good = vec![false; n];
    rec(&graph, None, 0, &cc, &mut counts, &mut good);

    println!("{}", (1..=n).filter(|&i| good[i - 1]).join("\n"));
}

fn rec(
    graph: &[Vec<usize>],
    par: Option<usize>,
    cur: usize,
    cc: &[usize],
    counts: &mut [usize],
    good: &mut [bool],
) {
    if counts[cc[cur]] == 0 {
        good[cur] = true;
    }

    counts[cc[cur]] += 1;

    for &next in &graph[cur] {
        if Some(next) != par {
            rec(&graph, Some(cur), next, cc, counts, good);
        }
    }

    counts[cc[cur]] -= 1;
}
