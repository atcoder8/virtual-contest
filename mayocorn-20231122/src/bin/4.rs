use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let primes1: HashSet<usize> = prime_factorization(a).iter().map(|x| x.0).collect();
    let primes2: HashSet<usize> = prime_factorization(b).iter().map(|x| x.0).collect();

    let ans = primes1.intersection(primes2).len() + 1;
    println!("{}", ans);
}

/// Performs prime factorization of `n`.
///
/// The result of the prime factorization is returned as a
/// list of prime factor and exponent pairs.
///
/// # Examples
///
/// ```
/// assert_eq!(prime_factorization(1), vec![]);
/// assert_eq!(prime_factorization(12), vec![(2, 2), (3, 1)]);
/// assert_eq!(prime_factorization(19), vec![(19, 1)]);
/// assert_eq!(prime_factorization(27), vec![(3, 3)]);
/// ```
pub fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut factors = vec![];
    let mut t = n;

    for p in 2.. {
        if p * p > t {
            break;
        }

        let mut e = 0;
        while t % p == 0 {
            t /= p;
            e += 1;
        }

        if e != 0 {
            factors.push((p, e));
        }
    }

    if t != 1 {
        factors.push((t, 1));
    }

    factors
}
