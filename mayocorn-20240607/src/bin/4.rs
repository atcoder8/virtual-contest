use proconio::input;

fn main() {
    input! {
        (n, t): (usize, String),
        ss: [String; n],
    }

    let mut prefix = vec![0_usize; t.len() + 1];
    for s in &ss {
        let len = count_prefix_len(s, &t);
        prefix[len] += 1;
    }

    let rev_t = t.chars().rev().collect::<String>();
    let mut suffix = vec![0_usize; t.len() + 1];
    for s in &ss {
        let rev_s = s.chars().rev().collect::<String>();
        let len = count_prefix_len(&rev_s, &rev_t);
        suffix[len] += 1;
    }

    for i in (0..t.len()).rev() {
        suffix[i] += suffix[i + 1];
    }

    let mut ans = 0;
    for i in 0..=t.len() {
        ans += prefix[i] * suffix[t.len() - i];
    }

    println!("{}", ans);
}

fn count_prefix_len(s: &str, t: &str) -> usize {
    let mut len = 0;
    let mut iter = s.chars();

    for c in t.chars() {
        match iter.find(|&v| v == c) {
            Some(_) => len += 1,
            None => break,
        }
    }

    len
}
