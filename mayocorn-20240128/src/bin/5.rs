use itertools::izip;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        s: Chars,
        t: Chars,
    }

    let mut middle = vec![false; n + 1];
    middle[0] = true;
    let mut end = vec![false; n + 1];
    for i in 0..n {
        if end[i] {
            for left in 0..m {
                let match_len = izip!(&s[i..], &t[left..])
                    .take_while(|(c1, c2)| c1 == c2)
                    .count();

                middle[i..=i + match_len].iter_mut().for_each(|x| *x = true);

                if left + match_len == m {
                    end[i + match_len] = true;
                }
            }
        } else if middle[i] {
            let match_len = izip!(&s[i..], &t).take_while(|(c1, c2)| c1 == c2).count();

            middle[i..=i + match_len].iter_mut().for_each(|x| *x = true);

            if match_len == m {
                end[i + m] = true;
            }
        }
    }

    println!("{}", if end[n] { "Yes" } else { "No" });
}
