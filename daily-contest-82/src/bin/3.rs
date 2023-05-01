use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [i64; n],
    }

    let mut neg = vec![];
    let mut non_neg = vec![];
    for (i, &a) in aa.iter().enumerate() {
        if a < 0 {
            neg.push(((-a as usize + i) / (i + 1), a, i));
        } else if a < n as i64 {
            non_neg.push((a, i));
        }
    }
    neg.sort_unstable_by_key(|x| Reverse(x.0));

    for step in 1..=m {
        let mut next_non_neg = vec![];
        for (val, i) in non_neg {
            let next_val = val + i as i64 + 1;
            if next_val < n as i64 {
                next_non_neg.push((next_val, i));
            }
        }
        non_neg = next_non_neg;

        while let Some(&(cnt, a, i)) = neg.last() {
            if cnt == step {
                neg.pop();
                let val = a + (step * (i + 1)) as i64;
                non_neg.push((val, i));
            } else {
                break;
            }
        }

        non_neg.sort_unstable_by_key(|x| x.0);

        let ans = if non_neg.is_empty() || non_neg[0].0 != 0 {
            0
        } else {
            let pos = (0..(non_neg.len() - 1)).find(|&i| non_neg[i + 1].0 - non_neg[i].0 >= 2);
            if let Some(pos) = pos {
                non_neg[pos].0 + 1
            } else {
                non_neg.last().unwrap().0 + 1
            }
        };
        println!("{}", ans);
    }
}
