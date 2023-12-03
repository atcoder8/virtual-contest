use im_rc::{HashMap, HashSet};
use itertools::{enumerate, Itertools};
use proconio::{input, marker::Chars};

use crate::eratosthenes_sieve::EratosthenesSieve;

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let max_s_len = ss.iter().map(|s| s.len()).max().unwrap();

    let sieve = EratosthenesSieve::new(max_s_len);

    let mut used_per_chars = HashMap::<Vec<char>, UsedLengths>::new();

    let mut kk = vec![0; n];
    for (i, s) in enumerate(&ss) {
        let z = z_algorithm(s);

        let is_ok = |len: usize| (0..s.len()).step_by(len).all(|start| z[start] >= len);

        let min_len = sieve
            .create_divisor_list(z.len())
            .into_iter()
            .find(|&len| is_ok(len))
            .unwrap();

        let used_lengths = used_per_chars.entry(s[..min_len].to_owned()).or_default();
        kk[i] = used_lengths.insert(s.len() / min_len);
    }

    println!("{}", kk.iter().join(" "));
}

#[derive(Debug, Default, Clone)]
struct UsedLengths {
    pool: HashSet<usize>,
    used_per_multiple: HashMap<usize, usize>,
}

impl UsedLengths {
    fn insert(&mut self, mul: usize) -> usize {
        let k = self.used_per_multiple.entry(mul).or_insert(1);
        while self.pool.contains(&(mul * *k)) {
            *k += 1;
        }

        self.pool.insert(mul * *k);

        *k
    }
}

pub fn z_algorithm<T>(seq: &[T]) -> Vec<usize>
where
    T: Eq,
{
    if seq.is_empty() {
        return vec![];
    }

    let n = seq.len();

    let mut z = vec![0; n];
    let mut j = 0;

    for i in 1..n {
        if j + z[j] > i {
            z[i] = z[i - j].min(j + z[j] - i);
        }

        let k = &mut z[i];

        while i + *k < n && seq[*k] == seq[i + *k] {
            *k += 1;
        }

        if j + z[j] < i + z[i] {
            j = i;
        }
    }

    z[0] = n;

    z
}

pub mod eratosthenes_sieve {
    //! Implements the Sieve of Eratosthenes.
    //!
    //! Finds the smallest prime factor of each number placed on the sieve,
    //! so it can perform Prime Factorization as well as Primality Test.

    #[derive(Debug, Clone)]
    pub struct EratosthenesSieve {
        sieve: Vec<usize>,
    }

    impl EratosthenesSieve {
        /// Constructs a Sieve of Eratosthenes.
        ///
        /// # Arguments
        ///
        /// * `upper_limit` - The largest number placed on the sieve.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
        /// ```
        pub fn new(upper_limit: usize) -> Self {
            let mut sieve: Vec<usize> = (0..=upper_limit).collect();

            for p in (2..).take_while(|&i| i * i <= upper_limit) {
                if sieve[p] != p {
                    continue;
                }

                for i in ((p * p)..=upper_limit).step_by(p) {
                    if sieve[i] == i {
                        sieve[i] = p;
                    }
                }
            }

            Self { sieve }
        }

        /// Returns the least prime factor of `n`.
        ///
        /// However, if `n` is `1`, then `1` is returned.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.get_least_prime_factor(1), 1);
        /// assert_eq!(sieve.get_least_prime_factor(2), 2);
        /// assert_eq!(sieve.get_least_prime_factor(6), 2);
        /// assert_eq!(sieve.get_least_prime_factor(11), 11);
        /// assert_eq!(sieve.get_least_prime_factor(27), 3);
        /// ```
        pub fn get_least_prime_factor(&self, n: usize) -> usize {
            assert_ne!(n, 0, "`n` must be at least 1.");

            self.sieve[n]
        }

        /// Determines if `n` is prime.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert!(!sieve.is_prime(1));
        /// assert!(sieve.is_prime(2));
        /// assert!(!sieve.is_prime(6));
        /// assert!(sieve.is_prime(11));
        /// assert!(!sieve.is_prime(27));
        /// ```
        pub fn is_prime(&self, n: usize) -> bool {
            n >= 2 && self.sieve[n] == n
        }

        /// Performs prime factorization of `n`.
        ///
        /// The result of the prime factorization is returned as a
        /// list of prime factor and exponent pairs.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.prime_factorization(1), vec![]);
        /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
        /// assert_eq!(sieve.prime_factorization(19), vec![(19, 1)]);
        /// assert_eq!(sieve.prime_factorization(27), vec![(3, 3)]);
        /// ```
        pub fn prime_factorization(&self, n: usize) -> Vec<(usize, usize)> {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let mut n = n;

            let mut factors: Vec<(usize, usize)> = vec![];

            while n != 1 {
                let p = self.sieve[n];

                if factors.is_empty() || factors.last().unwrap().0 != p {
                    factors.push((p, 1));
                } else {
                    factors.last_mut().unwrap().1 += 1;
                }

                n /= p;
            }

            factors
        }

        /// Creates a list of positive divisors of `n`.
        ///
        /// The positive divisors are listed in ascending order.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.create_divisor_list(1), vec![1]);
        /// assert_eq!(sieve.create_divisor_list(12), vec![1, 2, 3, 4, 6, 12]);
        /// assert_eq!(sieve.create_divisor_list(19), vec![1, 19]);
        /// assert_eq!(sieve.create_divisor_list(27), vec![1, 3, 9, 27]);
        /// ```
        pub fn create_divisor_list(&self, n: usize) -> Vec<usize> {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let prime_factors = self.prime_factorization(n);
            let divisor_num: usize = prime_factors.iter().map(|&(_, exp)| exp + 1).product();

            let mut divisors = vec![1];
            divisors.reserve(divisor_num - 1);

            for (p, e) in prime_factors {
                let mut add_divisors = vec![];
                add_divisors.reserve(divisors.len() * e);
                let mut mul = 1;

                for _ in 1..=e {
                    mul *= p;

                    for &d in divisors.iter() {
                        add_divisors.push(d * mul);
                    }
                }

                divisors.append(&mut add_divisors);
            }

            divisors.sort_unstable();

            divisors
        }

        /// Calculates the number of positive divisors of `n`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.calc_divisor_num(1), 1);
        /// assert_eq!(sieve.calc_divisor_num(12), 6);
        /// assert_eq!(sieve.calc_divisor_num(19), 2);
        /// assert_eq!(sieve.calc_divisor_num(27), 4);
        /// ```
        pub fn calc_divisor_num(&self, n: usize) -> usize {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let mut n = n;

            let mut divisor_num = 1;
            let mut cur_p = None;
            let mut cur_exp = 0;

            while n != 1 {
                let p = self.sieve[n];

                if Some(p) == cur_p {
                    cur_exp += 1;
                } else {
                    divisor_num *= cur_exp + 1;

                    cur_p = Some(p);
                    cur_exp = 1;
                }

                n /= p;
            }

            divisor_num *= cur_exp + 1;

            divisor_num
        }
    }
}
