use itertools::enumerate;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, p, q, r): (usize, usize, usize, usize),
        aa: [usize; n],
    }

    let mut prefix_sum = vec![0; n + 1];
    for (i, &a) in enumerate(&aa) {
        prefix_sum[i + 1] = prefix_sum[i] + a;
    }

    for x in 0..n {
        let Ok(y) = prefix_sum.binary_search(&(prefix_sum[x] + p)) else { continue; };
        let Ok(z) = prefix_sum.binary_search(&(prefix_sum[y] + q)) else { continue; };
        let Ok(_w) = prefix_sum.binary_search(&(prefix_sum[z] + r)) else { continue; };

        return true;
    }

    false
}
