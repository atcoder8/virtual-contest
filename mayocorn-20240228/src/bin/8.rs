use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
        abcd: [(Usize1, Usize1, Usize1, usize); n - 1],
        xyuv: [(Usize1, usize, Usize1, Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, c, d) in &abcd {
        let edge = Edge::new(u, v, c, d);
        graph[u].push(edge);
        graph[v].push(edge.rev());
    }

    enum DFSNode {
        Forward(Option<Edge>),
        Backward(Edge),
    }

    let mut dists = vec![0; n];
    let mut count_each_color = PrefixSumPerColor::new(n - 1);
    let mut sum_len_each_color = PrefixSumPerColor::new(n - 1);
    let mut parents: Vec<Option<usize>> = vec![None; n];
    let mut depths: Vec<usize> = vec![0; n];
    let mut order_each_node = vec![0; n];

    let mut stack: Vec<DFSNode> = vec![DFSNode::Forward(None)];
    let mut visited = vec![false; n];
    let mut order = 1;
    while let Some(dfs_node) = stack.pop() {
        match dfs_node {
            DFSNode::Forward(edge) => {
                let cur = edge.map_or(0, |edge| edge.to);

                if visited[cur] {
                    continue;
                }

                visited[cur] = true;

                if let Some(edge) = edge {
                    dists[cur] = dists[edge.from] + edge.length;
                    count_each_color.add(edge.color, 1, order);
                    sum_len_each_color.add(edge.color, edge.length, order);
                    parents[cur] = Some(edge.from);
                    depths[cur] = depths[edge.from] + 1;
                    order_each_node[cur] = order;

                    stack.push(DFSNode::Backward(edge));
                }

                stack.extend(
                    graph[cur]
                        .iter()
                        .map(|&next_edge| DFSNode::Forward(Some(next_edge))),
                );
            }

            DFSNode::Backward(edge) => {
                count_each_color.sub(edge.color, 1, order);
                sum_len_each_color.sub(edge.color, edge.length, order);
            }
        }

        order += 1;
    }

    let max_depth = *depths.iter().max().unwrap();
    let log_max_depth = (0_usize..).find(|&i| max_depth >> i == 1).unwrap();

    let mut doubling = vec![parents];
    for exp in 1..=log_max_depth {
        let prev = &doubling[exp - 1];
        let mut double = vec![None; n];
        for i in 1..n {
            double[i] = prev[i].map_or(None, |half| prev[half]);
        }
        doubling.push(double);
    }

    let find_lca = |mut u: usize, mut v: usize| {
        if depths[u] > depths[v] {
            std::mem::swap(&mut u, &mut v);
        }

        let diff = depths[v] - depths[u];
        for i in (0..).take_while(|&i| diff >> i != 0) {
            if diff >> i & 1 == 1 {
                v = doubling[i][v].unwrap();
            }
        }

        if u == v {
            return u;
        }

        for i in (0..=log_max_depth).rev() {
            if doubling[i][u] != doubling[i][v] {
                u = doubling[i][u].unwrap();
                v = doubling[i][v].unwrap();
            }
        }

        doubling[0][u].unwrap()
    };

    let solve = |x: usize, y: usize, u: usize, v: usize| {
        let lca = find_lca(u, v);

        let order_lca = order_each_node[lca];
        let order_u = order_each_node[u];
        let order_v = order_each_node[v];

        let sum_len = sum_len_each_color.sum(x, order_lca, order_u)
            + sum_len_each_color.sum(x, order_lca, order_v);
        let sum_count = count_each_color.sum(x, order_lca, order_u)
            + count_each_color.sum(x, order_lca, order_v);

        dists[u] + dists[v] - 2 * dists[lca] + sum_count * y - sum_len
    };

    let ans = xyuv
        .iter()
        .map(|&(x, y, u, v)| solve(x, y, u, v))
        .join("\n");
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    color: usize,
    length: usize,
}

impl Edge {
    fn new(from: usize, to: usize, color: usize, length: usize) -> Self {
        Self {
            from,
            to,
            color,
            length,
        }
    }

    fn rev(&self) -> Self {
        let mut rev_edge = *self;
        std::mem::swap(&mut rev_edge.from, &mut rev_edge.to);

        rev_edge
    }
}

#[derive(Debug, Clone)]
struct PrefixSumPerColor {
    prefix_sum: Vec<usize>,
    prefix_sum_per_color: Vec<Vec<(usize, usize)>>,
}

impl PrefixSumPerColor {
    fn new(color_num: usize) -> Self {
        Self {
            prefix_sum: vec![0; color_num],
            prefix_sum_per_color: vec![vec![(0, 0)]; color_num],
        }
    }

    fn add(&mut self, color: usize, val: usize, order: usize) {
        self.prefix_sum[color] += val;
        self.prefix_sum_per_color[color].push((order, self.prefix_sum[color]));
    }

    fn sub(&mut self, color: usize, val: usize, order: usize) {
        self.prefix_sum[color] -= val;
        self.prefix_sum_per_color[color].push((order, self.prefix_sum[color]));
    }

    fn sum(&self, color: usize, order1: usize, order2: usize) -> usize {
        let prefix_sum = &self.prefix_sum_per_color[color];

        let idx1 = prefix_sum.lower_bound(&(order1 + 1, 0));
        let idx2 = prefix_sum.lower_bound(&(order2 + 1, 0));

        if idx1 < idx2 {
            prefix_sum[idx2 - 1].1 - prefix_sum[idx1 - 1].1
        } else {
            0
        }
    }
}
