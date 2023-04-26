use proconio::{input, marker::Chars};

const BASE: [char; 3] = ['1', '1', '0'];
const REPEAT_NUM: usize = 10_000_000_000;

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("0");
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        t: Chars,
    }

    if t == vec!['0'] {
        return Some(REPEAT_NUM);
    }

    if t == vec!['1'] {
        return Some(2 * REPEAT_NUM);
    }

    let check_match = |shift: usize| (0..n).all(|i| t[i] == BASE[(i + shift) % 3]);

    let shift = (0..3).find(|&shift| check_match(shift))?;

    let tail = shift + n - 1;

    Some((3 * REPEAT_NUM - tail + 2) / 3)
}
