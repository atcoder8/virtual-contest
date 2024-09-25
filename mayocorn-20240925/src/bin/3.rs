use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        ss: [String; n],
    }

    let count_match_types = |bits: usize| {
        let mut appearances = [0; 26];
        for (i, s) in enumerate(&ss) {
            if bits >> i & 1 == 1 {
                s.chars().for_each(|c| appearances[char_to_int(c)] += 1);
            }
        }

        appearances.iter().filter(|&&cnt| cnt == k).count()
    };

    let ans = (0..1 << n).map(count_match_types).max().unwrap();
    println!("{}", ans);
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
