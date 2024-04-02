use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1..=n)
        .filter(|&i| i % 2 == 1 && find_divisors(i).len() == 8)
        .count();
    println!("{}", ans);
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
