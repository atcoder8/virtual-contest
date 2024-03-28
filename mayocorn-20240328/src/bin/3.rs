use im_rc::HashMap;
use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, i64),
        aa: [i64; n],
    }

    let mut prefix_sum = vec![0; n + 1];
    for (i, &a) in enumerate(&aa) {
        prefix_sum[i + 1] = prefix_sum[i] + a;
    }

    let mut ans = 0;
    let mut counts = HashMap::<i64, usize>::new();
    for &s in &prefix_sum {
        ans += counts.get(&(s - k)).cloned().unwrap_or_default();
        *counts.entry(s).or_default() += 1;
    }

    println!("{}", ans);
}
