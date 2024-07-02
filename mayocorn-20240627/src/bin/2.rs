use im_rc::HashMap;
use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [usize; n],
        xk: [(usize, usize); q],
    }

    let mut indexes_each_a = HashMap::<usize, Vec<usize>>::new();
    for (i, &a) in enumerate(&aa) {
        indexes_each_a.entry(a).or_default().push(i);
    }

    let ans = xk
        .iter()
        .map(|&(x, k)| {
            match indexes_each_a
                .get(&x)
                .and_then(|indexes| indexes.get(k - 1))
            {
                Some(&idx) => (idx + 1).to_string(),
                None => "-1".to_string(),
            }
        })
        .join("\n");
    println!("{}", ans);
}
