use fixedbitset::FixedBitSet;
use im_rc::HashSet;
use itertools::{iproduct, Itertools};
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        n: usize,
        k: usize,
        sss: [Chars; n],
    }

    let to_idx = |coord: (usize, usize)| coord.0 * n + coord.1;

    let mut ans = 0_usize;
    let mut pool = HashSet::<FixedBitSet>::new();
    let mut stack = (0..n.pow(2))
        .filter(|&idx| sss[idx / n][idx % n] == '.')
        .map(|idx| {
            let mut grid = FixedBitSet::with_capacity(n.pow(2));
            grid.insert(idx);

            grid
        })
        .collect_vec();
    while let Some(painted_grid) = stack.pop() {
        if pool.contains(&painted_grid) {
            continue;
        }

        pool.insert(painted_grid.clone());

        if painted_grid.count_ones(..) == k {
            ans += 1;
            continue;
        }

        for (row, col) in iproduct!(0..n, 0..n) {
            if !painted_grid[to_idx((row, col))] {
                continue;
            }

            for (diff_row, diff_col) in DIFFS {
                let adj_row = row.wrapping_add(diff_row);
                let adj_col = col.wrapping_add(diff_col);
                let adj_coord = (adj_row, adj_col);

                if adj_row < n
                    && adj_col < n
                    && sss[adj_row][adj_col] == '.'
                    && !painted_grid[to_idx(adj_coord)]
                {
                    let mut next_painted_grid = painted_grid.clone();
                    next_painted_grid.insert(to_idx(adj_coord));
                    stack.push(next_painted_grid);
                }
            }
        }
    }

    println!("{}", ans);
}
