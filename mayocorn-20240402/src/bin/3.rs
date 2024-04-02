use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut aa: [usize; n],
    }

    let mut each_rem = vec![vec![]; k];
    for (i, &a) in enumerate(&aa) {
        each_rem[i % k].push(a);
    }

    each_rem.iter_mut().for_each(|v| v.sort_unstable());

    let mut bb = vec![];
    for i in 0..n {
        bb.push(each_rem[i % k][i / k]);
    }

    aa.sort_unstable();
    let ans = bb == aa;
    println!("{}", if ans { "Yes" } else { "No" });
}
