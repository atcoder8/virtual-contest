use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        txc: [(usize, usize, char); q],
    }

    let mut edit_char_times = vec![0; n];
    let mut edit_case = None;

    for (i, &(t, x, c)) in enumerate(&txc) {
        let time = i + 1;

        match t {
            1 => {
                s[x - 1] = c;
                edit_char_times[x - 1] = time;
            }
            2 => {
                edit_case = Some((time, 'a'));
            }
            3 => {
                edit_case = Some((time, 'A'));
            }
            _ => unreachable!(),
        }
    }

    if let Some((edit_case_time, case)) = edit_case {
        izip!(&mut s, edit_char_times).for_each(|(c, edit_char_time)| {
            if edit_char_time < edit_case_time {
                if case == 'a' {
                    *c = c.to_ascii_lowercase();
                } else {
                    *c = c.to_ascii_uppercase();
                }
            }
        });
    }

    println!("{}", s.iter().join(""));
}
