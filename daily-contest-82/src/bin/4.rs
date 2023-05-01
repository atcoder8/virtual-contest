use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        wl: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    let mut indegree = vec![0_usize; n];
    for &(a, b) in &wl {
        graph[a].push(b);
        indegree[b] += 1;
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut ans = vec![];
    let mut stack = (0..n).filter(|&i| indegree[i] == 0).collect_vec();
    let mut unique = stack.len() == 1;

    while !stack.is_empty() {
        let mut next_stack = vec![];

        if stack.len() >= 2 {
            unique = false;
        }

        for &cur in &stack {
            for &next in &graph[cur] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    next_stack.push(next);
                }
            }
        }

        ans.append(&mut stack);

        stack = next_stack;
    }

    println!(
        "{}\n{}",
        join(ans.iter().map(|&x| x + 1), "\n"),
        !unique as usize
    );
}
