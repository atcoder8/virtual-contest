use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (_n, k): (usize, usize),
        s: Chars,
    }

    let mut positions = vec![vec![]; 3];
    for (i, &c) in s.iter().enumerate() {
        positions[char_to_usize(c)].push(i);
    }

    let find_i_end_pos = |j_start_pos: usize| {
        let mut end_pos = j_start_pos + k - 1;

        if end_pos >= positions[0].len() {
            return None;
        }

        for i in 1..3 {
            end_pos = positions[i].upper_bound(&positions[i - 1][end_pos]) + k - 1;

            if end_pos >= positions[i].len() {
                return None;
            }
        }

        Some(end_pos)
    };

    let mut ans = None;
    for j_start_pos in 0..positions[0].len() {
        match find_i_end_pos(j_start_pos) {
            Some(i_end_pos) => {
                let cost = positions[2][i_end_pos] - positions[0][j_start_pos] + 1 - 3 * k;
                if ans.is_none() || cost < ans.unwrap() {
                    ans = Some(cost);
                }
            }
            None => break,
        }
    }

    ans
}

fn char_to_usize(c: char) -> usize {
    match c {
        'J' => 0,
        'O' => 1,
        'I' => 2,
        _ => panic!(),
    }
}
