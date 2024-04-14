use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, usize); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvw {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let mut colors = vec![false; n];
    let mut stack: Vec<(Option<usize>, usize, usize)> = vec![(None, 0, 0)];
    while let Some((par, cur, dist)) = stack.pop() {
        colors[cur] = dist % 2 == 1;

        stack.extend(graph[cur].iter().filter_map(|&(next, weight)| {
            if Some(next) == par {
                return None;
            }

            Some((Some(cur), next, dist + weight))
        }));
    }

    println!("{}", colors.iter().map(|&color| color as usize).join("\n"));
}
