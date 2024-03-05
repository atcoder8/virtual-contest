use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        mut aaa: [[usize; w]; h],
    }

    let mut operations = vec![];
    for (row, col) in iproduct!(0..h - 1, 0..w) {
        if aaa[row][col] % 2 == 1 {
            aaa[row][col] -= 1;
            aaa[row + 1][col] += 1;

            operations.push((row, col, row + 1, col));
        }
    }

    for col in 0..w - 1 {
        if aaa[h - 1][col] % 2 == 1 {
            aaa[h - 1][col] -= 1;
            aaa[h - 1][col + 1] += 1;

            operations.push((h - 1, col, h - 1, col + 1));
        }
    }

    println!(
        "{}\n{}",
        operations.len(),
        operations
            .iter()
            .map(|op| format!("{} {} {} {}", op.0 + 1, op.1 + 1, op.2 + 1, op.3 + 1))
            .join("\n")
    );
}
