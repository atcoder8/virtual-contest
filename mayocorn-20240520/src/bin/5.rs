use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    println!("{}", solve(ab));
}

fn solve(ab: Vec<(usize, usize)>) -> usize {
    if ab.len() % 2 == 1 {
        solve_odd(ab)
    } else {
        solve_even(ab)
    }
}

fn solve_odd(mut ab: Vec<(usize, usize)>) -> usize {
    let n = ab.len();
    let mid = n / 2;

    ab.sort_unstable_by_key(|v| v.0);
    let min = ab[mid].0;

    ab.sort_unstable_by_key(|v| v.1);
    let max = ab[mid].1;

    max - min + 1
}

fn solve_even(mut ab: Vec<(usize, usize)>) -> usize {
    let n = ab.len();
    let mid = n / 2;

    ab.sort_unstable_by_key(|v| v.0);
    let min = ab[mid - 1].0 + ab[mid].0;

    ab.sort_unstable_by_key(|v| v.1);
    let max = ab[mid - 1].1 + ab[mid].1;

    max - min + 1
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{rngs::ThreadRng, thread_rng, Rng};

    use crate::solve;

    /// Input data type.
    type Input = Vec<(usize, usize)>;

    /// Output data type.
    type Output = usize;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        let mut rng = thread_rng();

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator(&mut rng);
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
    pub fn generator(rng: &mut ThreadRng) -> Input {
        let n = rng.gen_range(1..=7);
        let ab = (0..n)
            .map(|_| {
                let a = rng.gen_range(1..=8);
                let b = rng.gen_range(a..=8);
                (a, b)
            })
            .collect_vec();

        ab
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let ab = input;

        let n = ab.len();
        let mid = n / 2;

        let mut medians = vec![];
        let mut stack = vec![vec![]];

        while let Some(mut xx) = stack.pop() {
            if xx.len() == n {
                xx.sort_unstable();

                if n % 2 == 1 {
                    medians.push(xx[mid]);
                } else {
                    medians.push(xx[mid - 1] + xx[mid]);
                }

                continue;
            }

            let (a, b) = ab[xx.len()];
            stack.extend((a..=b).map(|x| {
                let mut next_xx = xx.clone();
                next_xx.push(x);

                next_xx
            }));
        }

        medians.iter().unique().count()
    }

    /// Test this program.
    pub fn actual(input: Input) -> Output {
        let ab = input;

        solve(ab)
    }
}
