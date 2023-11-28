use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = 0;
    let mut colors = vec![None; n];
    for start in 0..n {
        if colors[start].is_some() {
            continue;
        }

        let mut size = 0;
        let mut true_num = 0;
        let mut stack = vec![(start, false)];
        while let Some((cur, color)) = stack.pop() {
            if let Some(decided_color) = colors[cur] {
                if decided_color != color {
                    return 0;
                }

                continue;
            }

            size += 1;

            true_num += color as usize;

            colors[cur] = Some(color);

            for &next in &graph[cur] {
                stack.push((next, !color));
            }
        }

        ans += 2 * true_num * (size - true_num) + size * (n - size);
    }
    ans = ans / 2 - m;

    ans
}
