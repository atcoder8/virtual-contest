use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let s = aa[..2].iter().sum::<usize>();
    let x = aa[2..].iter().fold(0, |acc, x| acc ^ x);

    if s < x || (s - x) % 2 == 1 {
        return None;
    }

    let d = (s - x) / 2;

    if x & d != 0 || d > aa[0] {
        return None;
    }

    let mut a = d;
    for digit in (0..40).rev() {
        if x >> digit & 1 == 1 && a | 1 << digit <= aa[0] {
            a |= 1 << digit;
        }
    }

    if a == 0 {
        return None;
    }

    Some(aa[0] - a)
}
