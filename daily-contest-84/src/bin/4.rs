use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xe: [(usize, usize); n],
    }

    let mut receive_flags = vec![false; n];
    let mut xei = xe
        .iter()
        .enumerate()
        .map(|(i, &(x, e))| (x, e, i))
        .collect_vec();
    xei.sort_unstable();
    let mut xei2 = xei.clone();
    xei2.sort_unstable_by_key(|x| x.1);

    let mut idx_to_x = vec![0; n];
    for i in 0..n {
        idx_to_x[xei[i].2] = i;
    }

    let mut ans = 0;

    while let Some((x, e, k)) = xei2.pop() {
        let pos = idx_to_x[k];

        if receive_flags[k] {
            continue;
        }

        receive_flags[k] = true;

        for i in (0..pos).rev() {
            let (x2, e2, idx) = xei[i];

            if receive_flags[idx] || x - x2 > e - e2 {
                break;
            }

            receive_flags[idx] = true;
        }

        for i in (pos + 1)..n {
            let (x2, e2, idx) = xei[i];

            if receive_flags[idx] || x2 - x > e - e2 {
                break;
            }

            receive_flags[idx] = true;
        }

        ans += 1;
    }

    println!("{}", ans);
}
