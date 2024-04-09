use im_rc::HashMap;
use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [String; n],
        tt: [String; m],
    }

    let mut name_to_idx = HashMap::new();
    for (i, s) in enumerate(ss) {
        name_to_idx.insert(s, i);
    }

    let mut ans = vec![false; n];
    for t in tt {
        ans[name_to_idx[&t]] = true;
    }

    let ans = ans
        .iter()
        .map(|&visit| if visit { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}
