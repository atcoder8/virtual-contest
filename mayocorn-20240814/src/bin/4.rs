use num_integer::Roots;
use proconio::input;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
    }

    let find_ok = |a: usize| {
        let b = (m + a - 1) / a;

        if b > n {
            return None;
        }

        Some(a * b)
    };

    (1..=n.min(m.sqrt() + 1)).filter_map(find_ok).min()
}

/// Creates a sequence consisting of the divisors of `n`.
pub fn find_divisors(n: usize) -> Vec<usize> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut divisors = vec![];

    for i in (1..).take_while(|&i| i <= n / i) {
        if n % i == 0 {
            divisors.push(i);

            if n / i != i {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort_unstable();

    divisors
}
