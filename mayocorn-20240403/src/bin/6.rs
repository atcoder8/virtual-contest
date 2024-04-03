use im_rc::hashset;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut xy: [(usize, usize); m],
    }

    xy.sort_unstable();

    let mut ys = hashset![n];
    for (_, groups) in &xy.iter().group_by(|v| v.0) {
        let mut add_ys = vec![];

        let black_ys = groups.map(|v| v.1).collect_vec();

        for &black_y in &black_ys {
            if black_y > 0 && ys.contains(&(black_y - 1))
                || black_y < 2 * n - 1 && ys.contains(&(black_y + 1))
            {
                add_ys.push(black_y);
            }
        }

        for &black_y in &black_ys {
            ys.remove(&black_y);
        }

        ys.extend(add_ys);
    }

    println!("{}", ys.len());
}
