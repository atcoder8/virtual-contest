use itertools::Itertools;
use maplit::hashset;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut xy: [(usize, usize); m],
    }

    xy.sort_unstable_by_key(|x| x.0);

    let mut set = hashset! {n};
    for (_, group) in &xy.iter().group_by(|x| x.0) {
        let yy = group.collect_vec();

        let mut add_set = hashset! {};
        for &&(_, y) in &yy {
            if (y > 0 && set.contains(&(y - 1))) || set.contains(&(y + 1)) {
                add_set.insert(y);
            }
        }

        for &&(_, y) in &yy {
            set.remove(&y);
        }

        set.extend(add_set);
    }

    println!("{}", set.len());
}
