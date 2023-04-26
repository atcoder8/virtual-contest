use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        (_n, k): (usize, usize),
        s: Chars,
    }

    let mut indexes = vec![vec![]; 3];
    for (i, &c) in s.iter().enumerate() {
        match c {
            'J' => indexes[0].push(i),
            'O' => indexes[1].push(i),
            'I' => indexes[2].push(i),
            _ => panic!(),
        }
    }

    let mut ans = None;

    for left_j in 0..indexes[0].len() {
        let right_j = left_j + k - 1;
        if right_j >= indexes[0].len() {
            break;
        }

        let left_o = indexes[1].upper_bound(&indexes[0][right_j]);
        let right_o = left_o + k - 1;
        if right_o >= indexes[1].len() {
            break;
        }

        let left_i = indexes[2].upper_bound(&indexes[1][right_o]);
        let right_i = left_i + k - 1;
        if right_i >= indexes[2].len() {
            break;
        }

        let cost = indexes[2][right_i] - indexes[0][left_j] + 1 - 3 * k;

        if ans.is_none() || cost < ans.unwrap() {
            ans = Some(cost);
        }
    }

    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
