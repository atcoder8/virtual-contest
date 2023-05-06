use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut si = ss
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect_vec();
    si.sort_unstable();

    let mut ans = vec![0; n];
    for i in 0..n {
        let idx = si[i].1;

        if i > 0 {
            ans[idx] = ans[idx].max(
                si[i]
                    .0
                    .iter()
                    .zip(&si[i - 1].0)
                    .take_while(|(c1, c2)| c1 == c2)
                    .count(),
            );
        }
        if i < n - 1 {
            ans[idx] = ans[idx].max(
                si[i]
                    .0
                    .iter()
                    .zip(&si[i + 1].0)
                    .take_while(|(c1, c2)| c1 == c2)
                    .count(),
            );
        }
    }

    println!("{}", ans.iter().map(|x| x.to_string()).join("\n"));
}
