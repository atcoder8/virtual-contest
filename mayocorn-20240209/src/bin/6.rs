// unfinished

use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    println!("{}", if solve(&ss) { "Yes" } else { "No" });
}

fn solve(ss: &[Vec<char>]) -> bool {
    let n = ss.len();

    let mut levels = ss
        .iter()
        .map(|s| calc_level(s))
        .sorted_by_key(|x| x.1)
        .collect_vec();

    let mut heap = BinaryHeap::new();
    let mut cur_level = 0;
    for _ in 0..n {
        while let Some(&(add_level, min_level)) = levels.last() {
            if cur_level + min_level >= 0 {
                levels.pop();
                heap.push((add_level, min_level));
            } else {
                break;
            }
        }

        let Some((add_level, min_level)) = heap.pop() else { return false; };

        if cur_level + min_level < 0 {
            return false;
        }

        cur_level += add_level;
    }

    cur_level == 0
}

fn calc_level(s: &[char]) -> (i64, i64) {
    let mut cur_level = 0;
    let mut min_level = 0;
    for &c in s {
        if c == '(' {
            cur_level += 1;
        } else {
            cur_level -= 1;

            min_level = min_level.min(cur_level);
        }
    }

    (cur_level, min_level)
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::Rng;

    use crate::calc_level;

    /// Input data type.
    type Input = Vec<Vec<char>>;

    /// Output data type.
    type Output = bool;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let expected_output = expected(input.clone());
            let actual_output = solve(input.clone());

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

        let n = rng.gen_range(1..=3);

        (0..n)
            .map(|_| {
                let len = rng.gen_range(1..=3);

                (0..len)
                    .map(|_| if rng.gen_bool(0.5) { '(' } else { ')' })
                    .collect()
            })
            .collect()
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let ss = input;
        let n = ss.len();

        let is_ok = |perm: &[usize]| {
            let combined = perm
                .iter()
                .map(|&i| &ss[i])
                .flatten()
                .cloned()
                .collect_vec();

            let (sum_level, min_level) = calc_level(&combined);

            sum_level == 0 && min_level == 0
        };

        (0..n).permutations(n).any(|perm| is_ok(&perm))
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let ss = input;

        crate::solve(&ss)
    }
}
