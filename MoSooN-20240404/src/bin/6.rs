use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    }

    println!("{}", solve(ab));
}

fn solve(ab: Vec<(i64, i64)>) -> i64 {
    let mut max_x = 0;
    let mut cur = 0;

    for &(a, b) in ab
        .iter()
        .filter(|(a, b)| a < b)
        .sorted_unstable_by_key(|(a, _b)| a)
    {
        max_x = max_x.max(cur + a);
        cur += a - b;
    }

    for &(a, b) in ab
        .iter()
        .filter(|(a, b)| a >= b)
        .sorted_unstable_by_key(|(_a, b)| Reverse(b))
    {
        max_x = max_x.max(cur + a);
        cur += a - b;
    }

    max_x
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::Rng;

    use crate::solve;

    /// Input data type.
    type Input = Vec<(i64, i64)>;

    /// Output data type.
    type Output = i64;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let expected_output = expected(input.clone());
            let actual_output = actual(input.clone());

            assert_eq!(
                expected_output, actual_output,
                "
Wrong Answer on Test #{}

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
                test_case_index, input, expected_output, actual_output
            );
        }
    }

    /// Generate test cases.
    pub fn generator() -> Input {
        let mut rng = rand::thread_rng();

        let n = rng.gen_range(1..=8);
        let ab = (0..n)
            .map(|_| (rng.gen_range(1..=10), rng.gen_range(1..=10)))
            .collect_vec();

        ab
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let ab = input;

        let calc_max_x = |perm: &[usize]| {
            let mut max_x = 0;
            let mut cur = 0;

            for &i in perm {
                let (a, b) = ab[i];

                max_x = max_x.max(cur + a);
                cur += a - b;
            }

            max_x
        };

        (0..ab.len())
            .permutations(ab.len())
            .map(|perm| calc_max_x(&perm))
            .min()
            .unwrap()
    }

    /// Test this program.
    pub fn actual(input: Input) -> Output {
        let ab = input;

        solve(ab)
    }
}
