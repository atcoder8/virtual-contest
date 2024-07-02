use itertools::izip;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut left = vec![0_usize; 26];
    let mut right = vec![0_usize; 26];
    for &c in &s {
        right[char_to_int(c)] += 1;
    }

    let mut ans = 0;
    for &c in &s {
        let c = char_to_int(c);
        left[c] += 1;
        right[c] -= 1;

        let num = izip!(&left, &right)
            .filter(|&(&cnt1, &cnt2)| cnt1 != 0 && cnt2 != 0)
            .count();
        ans = ans.max(num);
    }

    println!("{}", ans);
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
