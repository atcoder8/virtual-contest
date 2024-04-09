use itertools::izip;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, u, v): (usize, Usize1, Usize1),
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let calc_dists = |start: usize| {
        let mut dists = vec![0_usize; n];
        let mut stack: Vec<(Option<usize>, usize)> = vec![(None, start)];
        while let Some((par, cur)) = stack.pop() {
            dists[cur] = par.map(|par| dists[par] + 1).unwrap_or(0);
            stack.extend(
                graph[cur]
                    .iter()
                    .filter(|&&next| Some(next) != par)
                    .map(|&next| (Some(cur), next)),
            );
        }

        dists
    };

    let dists_from_u = calc_dists(u);
    let dists_from_v = calc_dists(v);

    let ans = izip!(&dists_from_u, &dists_from_v)
        .filter(|(dist1, dist2)| dist1 < dist2)
        .map(|(_, &dist2)| dist2 - 1)
        .max()
        .unwrap();
    println!("{}", ans);
}
