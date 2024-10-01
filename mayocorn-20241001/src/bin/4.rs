use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(u, v)) in enumerate(&ab) {
        graph[u].push((v, i));
        graph[v].push((u, i));
    }

    let mut colors = vec![0_u32; n - 1];

    let mut stack = vec![(None::<usize>, 0_usize, 0_u32)];
    while let Some((par, cur, prev_color)) = stack.pop() {
        let mut color = 0_u32;
        for &(adj, edge_idx) in &graph[cur] {
            if par.is_some_and(|par| par == adj) {
                continue;
            }

            color += 1;
            if color == prev_color {
                color += 1;
            }

            colors[edge_idx] = color;
            stack.push((Some(cur), adj, color));
        }
    }

    let num_colors = *colors.iter().max().unwrap();
    let str_colors = colors.iter().join("\n");
    println!("{}\n{}", num_colors, str_colors);
}
