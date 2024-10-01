use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

const TRANSITION: [[usize; 2]; 3] = [[1, 2], [2, 0], [0, 1]];

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    }

    let solve = |mut t: usize, mut k: usize| {
        let init_pos: usize;
        if t < 63 {
            init_pos = k >> t;
            k -= init_pos << t;
        } else {
            init_pos = 0;
            t = t % 63 + 63;
            while t >= 63 || t >= 3 && k >> (t - 3) == 0 {
                t -= 3;
            }
        }

        let mut c = char_to_int(s[init_pos]);
        for i in (0_usize..t).rev() {
            c = TRANSITION[c][k >> i & 1];
        }

        (c as u8 + b'A') as char
    };

    let ans = tk.iter().map(|&(t, k)| solve(t, k)).join("\n");
    println!("{}", ans);
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'A') as usize
}
