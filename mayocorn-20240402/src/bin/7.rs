use itertools::{enumerate, Itertools};
use proconio::input;

use crate::scc::SCC;

fn main() {
    input! {
        (h, w): (usize, usize),
        mut aaa: [[usize; w]; h],
    }

    println!("{}", if solve(aaa) { "Yes" } else { "No" });
}

fn solve(mut aaa: Vec<Vec<usize>>) -> bool {
    let h = aaa.len();
    let w = aaa[0].len();

    aaa.retain(|aa| aa.iter().any(|&a| a != 0));

    aaa.sort_by_cached_key(|aa| {
        let min = *aa.iter().filter(|&&a| a != 0).min().unwrap();
        let max = *aa.iter().max().unwrap();

        (min, max)
    });

    if aaa.iter().tuple_windows().any(|(aa1, aa2)| {
        aa1.iter().max().unwrap() > aa2.iter().filter(|&&a| a != 0).min().unwrap()
    }) {
        return false;
    }

    let mut scc_graph = SCC::new(w + (h + 1) * (w + 1));
    let mut progress = 0;
    for aa in &aaa {
        for (_, group) in &enumerate(aa)
            .filter(|v| *v.1 != 0)
            .sorted_unstable_by_key(|v| v.1)
            .group_by(|v| v.1)
        {
            for (col, _) in group {
                scc_graph.add_edge(w + progress, col);
                scc_graph.add_edge(col, w + progress + 1);
            }

            progress += 1;
        }

        progress += 1;
    }

    scc_graph.scc().iter().all(|comp| comp.len() == 1)
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::{iproduct, Itertools};
    use rand::Rng;

    use crate::solve;

    /// Input data type.
    type Input = Vec<Vec<usize>>;

    /// Output data type.
    type Output = bool;

    const MAX_H: usize = 5;
    const MAX_W: usize = 5;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let expected_output = expected(input.clone());
            let actual_output = actual(input.clone());

            assert_eq!(
                expected_output, actual_output,
                "
Wrong Answer on Test #{}

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
                test_case_index, input, expected_output, actual_output
            );
        }
    }

    /// Generate test cases.
    pub fn generator() -> Input {
        let mut rng = rand::thread_rng();

        let (h, w) = loop {
            let h = rng.gen_range(1..=MAX_H);
            let w = rng.gen_range(1..=MAX_W);

            if h * w >= 2 {
                break (h, w);
            }
        };

        let mut aaa = vec![vec![0; w]; h];
        for (row, col) in iproduct!(0..h, 0..w) {
            aaa[row][col] = rng.gen_range(0..=h * w);
        }

        aaa
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let aaa = input;

        let h = aaa.len();
        let w = aaa[0].len();

        let is_ok = |row_orders: &[usize], col_orders: &[usize]| {
            let mut prev = 0;
            for (&row, &col) in iproduct!(row_orders, col_orders) {
                let a = aaa[row][col];

                if a == 0 {
                    continue;
                }

                if a < prev {
                    return false;
                }

                prev = a;
            }

            true
        };

        iproduct!((0..h).permutations(h), (0..w).permutations(w))
            .any(|(row_orders, col_orders)| is_ok(&row_orders, &col_orders))
    }

    /// Test this program.
    pub fn actual(input: Input) -> Output {
        let aaa = input;

        solve(aaa)
    }
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
