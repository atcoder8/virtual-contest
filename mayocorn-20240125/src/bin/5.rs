use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, q): (usize, usize),
        mut lr: [(Usize1, usize); q],
    }

    let mut graph = vec![vec![]; n + 1];
    for &(u, v) in &lr {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n + 1];
    let mut stack = vec![0];
    while let Some(cur) = stack.pop() {
        if visited[cur] {
            continue;
        }

        visited[cur] = true;

        stack.extend(graph[cur].clone());
    }

    visited[n]
}
