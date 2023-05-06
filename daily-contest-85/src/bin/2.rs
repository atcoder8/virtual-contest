use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut outdegree = vec![0; n];
    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        outdegree[a] += 1;
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut stack = (0..n).filter(|&i| outdegree[i] == 0).collect_vec();
    let mut labeled = vec![false; n];

    while let Some(cur) = stack.pop() {
        if labeled[cur] {
            continue;
        }

        labeled[cur] = true;

        for &next in &graph[cur] {
            outdegree[next] -= 1;
            if outdegree[next] == 0 {
                stack.push(next);
            }
        }
    }

    let ans = (0..n).filter(|&i| !labeled[i]).count();
    println!("{}", ans);
}
