use itertools::{izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans.iter().join("\n")),
        None => println!("UNSOLVABLE"),
    }
}

fn solve() -> Option<Vec<usize>> {
    input! {
        ss: [Chars; 3],
    }

    let mut labels: Vec<Option<usize>> = vec![None; 26];
    let mut cnt = 0;
    for s in &ss {
        for &c in s {
            let label = &mut labels[char_to_int(c)];

            if label.is_some() {
                continue;
            }

            if cnt == 10 {
                return None;
            }

            *label = Some(cnt);
            cnt += 1;
        }
    }

    (0..10).permutations(cnt).find_map(|perm| {
        let s_to_n = |s: &[char]| {
            let mut n = 0;
            for &c in s {
                let label = labels[char_to_int(c)].unwrap();
                n = 10 * n + perm[label];
            }

            n
        };

        let nn = ss.iter().map(|s| s_to_n(s)).collect_vec();

        if nn[0] + nn[1] != nn[2]
            || izip!(&ss, &nn).any(|(s, &n)| n == 0 || n.to_string().len() != s.len())
        {
            return None;
        }

        Some(nn)
    })
}

/// Converts the character `c` to the corresponding numeric value.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
