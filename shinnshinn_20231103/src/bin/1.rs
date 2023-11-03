use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
    }

    let mut visited = vec![false; n];
    let mut flags = vec![None; n];
    for start in 0..n {
        dfs(&graph, &mut visited, &mut flags, start);
    }

    let ans = flags.iter().filter(|&flag| flag.unwrap()).count();
    println!("{}", ans);
}

fn dfs(graph: &[Vec<usize>], visited: &mut [bool], flags: &mut [Option<bool>], cur: usize) -> bool {
    if let Some(is_match) = flags[cur] {
        return is_match;
    }

    if visited[cur] {
        flags[cur] = Some(true);

        return true;
    }

    visited[cur] = true;

    let is_match = graph[cur]
        .iter()
        .any(|&next| dfs(graph, visited, flags, next));
    flags[cur] = Some(is_match);

    is_match
}
