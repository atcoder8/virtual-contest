use itertools::Itertools;
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

    let mut subtree_size_vec = vec![0; n];
    let mut dists = vec![0; n];
    find_subtree_size(&graph, None, 0, 0, &mut subtree_size_vec, &mut dists);

    let sum_dist = dists.iter().sum::<usize>();

    let mut sum_dists = vec![0; n];
    find_sum_dists(&graph, None, 0, &subtree_size_vec, sum_dist, &mut sum_dists);

    println!("{}", sum_dists.iter().join("\n"));
}

fn find_subtree_size(
    graph: &[Vec<usize>],
    par: Option<usize>,
    cur: usize,
    dist: usize,
    subtree_size_vec: &mut [usize],
    dists: &mut [usize],
) -> usize {
    dists[cur] = dist;

    subtree_size_vec[cur] = graph[cur]
        .iter()
        .filter(|&&next| Some(next) != par)
        .map(|&next| find_subtree_size(graph, Some(cur), next, dist + 1, subtree_size_vec, dists))
        .sum::<usize>()
        + 1;

    subtree_size_vec[cur]
}

fn find_sum_dists(
    graph: &[Vec<usize>],
    par: Option<usize>,
    cur: usize,
    subtree_size_vec: &[usize],
    sum_dist: usize,
    sum_dists: &mut [usize],
) {
    sum_dists[cur] = sum_dist;

    for &next in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        find_sum_dists(
            graph,
            Some(cur),
            next,
            subtree_size_vec,
            sum_dist + graph.len() - 2 * subtree_size_vec[next],
            sum_dists,
        );
    }
}
