use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        ab: [(Usize1, Usize1); n - 1],
        cd: [(Usize1, Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut depths: Vec<Option<usize>> = vec![None; n];
    let mut stack = vec![(0, 0)];
    while let Some((cur, depth)) = stack.pop() {
        if depths[cur].is_some() {
            continue;
        }

        depths[cur] = Some(depth);

        stack.extend(graph[cur].iter().map(|&next| (next, depth + 1)));
    }

    let ans = cd
        .iter()
        .map(|&(c, d)| {
            if depths[c].unwrap().abs_diff(depths[d].unwrap()) % 2 == 0 {
                "Town"
            } else {
                "Road"
            }
        })
        .join("\n");
    println!("{}", ans);
}
