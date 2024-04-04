use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, k, c): (usize, usize, usize),
        s: Chars,
    }

    let mut backward = vec![0; n + 1];
    {
        let mut pos = n - 1;
        let mut cnt = 0;

        loop {
            while s[pos] == 'x' {
                pos -= 1;
            }

            cnt += 1;
            backward[pos] = cnt;

            if cnt == k {
                break;
            }

            pos -= c + 1;
        }
    }
    for i in (0..n).rev() {
        backward[i] = backward[i].max(backward[i + 1]);
    }

    let mut ans = vec![];
    {
        let mut pos = 0;
        let mut cnt = 0;

        loop {
            while s[pos] == 'x' {
                pos += 1;
            }

            if cnt + backward[pos + 1] < k {
                ans.push(pos + 1);
            }

            cnt += 1;

            if cnt == k {
                break;
            }

            pos += c + 1;
        }
    }

    println!("{}", ans.iter().join("\n"));
}
