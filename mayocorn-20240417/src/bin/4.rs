use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        pp: [Usize1; n - 1],
        xy: [(Usize1, usize); m],
    }

    let mut children = vec![vec![]; n];
    for (i, &p) in enumerate(&pp) {
        children[p].push(i + 1);
    }

    let mut levels = vec![0; n];
    for &(x, y) in &xy {
        levels[x] = levels[x].max(y + 1);
    }

    let mut stack: Vec<(Option<usize>, usize)> = vec![(None, 0)];
    while let Some((par, cur)) = stack.pop() {
        levels[cur] = levels[cur].max(par.map(|par| levels[par].saturating_sub(1)).unwrap_or(0));
        stack.extend(children[cur].iter().map(|&child| (Some(cur), child)));
    }

    eprintln!("levels = {:?}", levels);

    let ans = levels.iter().filter(|&&level| level != 0).count();
    println!("{}", ans);
}
