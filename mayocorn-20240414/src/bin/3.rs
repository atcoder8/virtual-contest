use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut counts = vec![0; 26];
    for c in s.chars() {
        counts[char_to_number(c)] += 1;
    }

    let ans = counts
        .iter()
        .tuple_combinations()
        .map(|(cnt1, cnt2)| cnt1 * cnt2)
        .sum::<usize>()
        + counts.iter().any(|&cnt| cnt >= 2) as usize;
    println!("{}", ans);
}

/// Converts the character `c` to the corresponding numeric value.
pub fn char_to_number(c: char) -> usize {
    (c as u8 - b'a') as usize
}
