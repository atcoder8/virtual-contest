use itertools::{chain, Itertools};
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

fn main() {
    let answer = match solve() {
        Some(value) => {
            let str_matrix = value
                .axis_iter(Axis(0))
                .map(|line| line.iter().join(""))
                .join("\n");
            format!("Yes\n{str_matrix}")
        }
        None => "No".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<Array2<char>> {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }

    let is_ok = |matrix: &Array2<char>, strict: bool| {
        for row in 0..n {
            let left_abc = find_left_abc(matrix, row);

            if left_abc.is_some_and(|ch| ch != r[row]) {
                return false;
            }

            if strict && left_abc.is_none() {
                return false;
            }

            if !matrix
                .slice(s![row, ..])
                .iter()
                .filter(|&&ch| ch != '.')
                .all_unique()
            {
                return false;
            }

            if strict
                && matrix
                    .slice(s![row, ..])
                    .iter()
                    .filter(|&&ch| ch == '.')
                    .count()
                    != n - 3
            {
                return false;
            }
        }

        for col in 0..n {
            let top_abc = find_top_abc(matrix, col);

            if top_abc.is_some_and(|ch| ch != c[col]) {
                return false;
            }

            if strict && top_abc.is_none() {
                return false;
            }

            if !matrix
                .slice(s![.., col])
                .iter()
                .filter(|&&ch| ch != '.')
                .all_unique()
            {
                return false;
            }

            if strict
                && matrix
                    .slice(s![.., col])
                    .iter()
                    .filter(|&&ch| ch == '.')
                    .count()
                    != n - 3
            {
                return false;
            }
        }

        true
    };

    let init_matrix = Array2::from_elem((n, n), '.');
    let mut stack = vec![(init_matrix, 0_usize)];
    while let Some((matrix, row)) = stack.pop() {
        if row == n {
            if is_ok(&matrix, true) {
                return Some(matrix);
            }

            continue;
        }

        for line in chain!("ABC".chars(), vec!['.'; n - 3]).permutations(n) {
            let mut next_matrix = matrix.clone();
            for col in 0..n {
                next_matrix[(row, col)] = line[col];
            }

            if !is_ok(&next_matrix, false) {
                continue;
            }

            stack.push((next_matrix, row + 1));
        }
    }

    None
}

fn find_left_abc(matrix: &Array2<char>, row: usize) -> Option<char> {
    matrix
        .slice(s![row, ..])
        .iter()
        .find(|&&ch| ch != '.')
        .cloned()
}

fn find_top_abc(matrix: &Array2<char>, col: usize) -> Option<char> {
    matrix
        .slice(s![.., col])
        .iter()
        .find(|&&ch| ch != '.')
        .cloned()
}
