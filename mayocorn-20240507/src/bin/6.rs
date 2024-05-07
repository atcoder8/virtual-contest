use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); n],
    }

    let mut pos_each_val = vec![vec![]; m];
    for (i, &(a, b)) in enumerate(&ab) {
        pos_each_val[a].push(i);
        pos_each_val[b].push(i);
    }

    let mut imos = vec![0; m + 2];
    let mut counts = vec![0; n];
    let mut contain_num = 0;
    let mut right = 0;

    for left in 0..m {
        while right < m && contain_num < n {
            for &pos in &pos_each_val[right] {
                counts[pos] += 1;

                if counts[pos] == 1 {
                    contain_num += 1;
                }
            }

            right += 1;
        }

        if contain_num < n {
            break;
        }

        imos[right - left] += 1;
        imos[m - left + 1] -= 1;

        for &pos in &pos_each_val[left] {
            counts[pos] -= 1;

            if counts[pos] == 0 {
                contain_num -= 1;
            }
        }
    }

    for i in 0..=m {
        imos[i + 1] += imos[i];
    }

    println!("{}", imos[1..=m].iter().join(" "));
}
