use itertools::rev;
use proconio::{input, marker::Usize1};

use crate::scc::SCC;

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    let mut scc_graph = SCC::new(n);
    for &(u, v) in &uv {
        graph[u].push(v);
        scc_graph.add_edge(u, v);
    }

    let scc = scc_graph.scc();

    let mut endless = vec![false; n];
    for comp in rev(&scc) {
        if comp.len() == 1 {
            endless[comp[0]] = graph[comp[0]].iter().any(|&next| endless[next]);
        } else {
            for &node in comp {
                endless[node] = true;
            }
        }
    }

    let ans = endless.iter().filter(|&&x| x).count();
    println!("{}", ans);
}

pub mod scc {
    #[derive(Debug, Clone)]
    pub struct SCC {
        graph: Vec<Vec<usize>>,
        inv_graph: Vec<Vec<usize>>,
    }

    impl SCC {
        pub fn new(n: usize) -> Self {
            Self {
                graph: vec![vec![]; n],
                inv_graph: vec![vec![]; n],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.graph[from].push(to);
            self.inv_graph[to].push(from);
        }

        pub fn scc(&self) -> Vec<Vec<usize>> {
            let n = self.graph.len();

            let mut order = vec![];
            let mut visited = vec![false; n];
            for start_node in 0..n {
                if !visited[start_node] {
                    order.append(&mut post_order_traversal(
                        &self.graph,
                        &mut visited,
                        start_node,
                    ));
                }
            }

            let mut scc = vec![];
            let mut visited = vec![false; n];
            for &start_node in order.iter().rev() {
                if !visited[start_node] {
                    scc.push(post_order_traversal(
                        &self.inv_graph,
                        &mut visited,
                        start_node,
                    ));
                }
            }

            scc
        }
    }

    fn post_order_traversal(
        graph: &[Vec<usize>],
        visited: &mut [bool],
        start_node: usize,
    ) -> Vec<usize> {
        let mut post_order = vec![];

        let mut stack = vec![(start_node, false)];

        while let Some((node, back)) = stack.pop() {
            if back {
                post_order.push(node);
            }

            if visited[node] {
                continue;
            }

            visited[node] = true;

            stack.push((node, true));

            stack.extend(graph[node].iter().map(|&next_node| (next_node, false)));
        }

        post_order
    }
}
