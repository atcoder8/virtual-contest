use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut lr = vec![(0, 0); n];
    rec(&graph, None, 0, 0, &mut lr);

    for &(l, r) in &lr {
        println!("{} {}", l + 1, r);
    }
}

fn rec(
    graph: &[Vec<usize>],
    par: Option<usize>,
    cur: usize,
    left: usize,
    lr: &mut [(usize, usize)],
) -> usize {
    let mut right = left;

    for &next in &graph[cur] {
        if Some(next) != par {
            right = rec(graph, Some(cur), next, right, lr);
        }
    }

    right += (left == right) as usize;

    lr[cur] = (left, right);

    right
}
