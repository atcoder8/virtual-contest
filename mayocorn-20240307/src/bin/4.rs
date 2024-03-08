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

    let mut parities: Vec<Option<bool>> = vec![None; n];
    for start in 0..n {
        if parities[start].is_some() {
            continue;
        }

        let mut odd = 0;
        let mut even = 0;

        let mut stack = vec![(start, false)];
        while let Some((cur, cand_parity)) = stack.pop() {
            if let Some(parity) = parities[cur] {
                if parity != cand_parity {
                    return 0;
                }

                continue;
            }

            parities[cur] = Some(cand_parity);

            if cand_parity {
                odd += 1;
            } else {
                even += 1;
            }

            stack.extend(graph[cur].iter().map(|&next| (next, !cand_parity)));
        }

        let group_size = odd + even;
        ans += 2 * odd * even + group_size * (n - group_size);
    }
    ans /= 2;
    ans -= m;

    ans
}
