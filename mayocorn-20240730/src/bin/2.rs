use std::collections::VecDeque;

use itertools::izip;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, m): (usize, usize),
        s: Chars,
        cc: [Usize1; n],
    }

    let mut chars_per_color = vec![VecDeque::new(); m];
    for (&ch, &color) in izip!(&s, &cc) {
        chars_per_color[color].push_back(ch);
    }
    chars_per_color.iter_mut().for_each(|chars| {
        let last = chars.pop_back().unwrap();
        chars.push_front(last);
    });

    let ans = cc
        .iter()
        .map(|&c| chars_per_color[c].pop_front().unwrap())
        .collect::<String>();
    println!("{}", ans);
}
