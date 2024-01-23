use proconio::input;

fn main() {
    let (a, b) = solve();
    println!("{a} {b}");
}

fn solve() -> (i64, i64) {
    input! {
        x: i64,
    }

    let divisors = find_divisors(x as usize);

    for a in 1.. {
        for &divisor in &divisors {
            let b = a - divisor as i64;
            if a.pow(5) - b.pow(5) == x {
                return (a, b);
            }
        }
    }

    unreachable!()
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
