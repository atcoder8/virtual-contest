use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let total_node_num = n * (n + 1) * (n + 2) / 6;
    let total_edge_num = uv
        .iter()
        .map(|&(u, v)| (u.min(v) + 1) * (n - u.max(v)))
        .sum::<usize>();
    let ans = total_node_num - total_edge_num;
    println!("{}", ans);
}
