use itertools::iproduct;
use proconio::{input, marker::Chars};

const CODE_SIZE: usize = 9;

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let is_ok = |top: usize, left: usize| {
        let get_color = |row: usize, col: usize| {
            return ss[top + row][left + col];
        };

        for (i, j) in iproduct!(0..4, 0..4) {
            if i < 3 && j < 3 {
                if get_color(i, j) != '#' || get_color(CODE_SIZE - 1 - i, CODE_SIZE - 1 - j) != '#'
                {
                    return false;
                }
            } else {
                if get_color(i, j) != '.' || get_color(CODE_SIZE - 1 - i, CODE_SIZE - 1 - j) != '.'
                {
                    return false;
                }
            }
        }

        true
    };

    let ans = iproduct!(0..n - CODE_SIZE + 1, 0..m - CODE_SIZE + 1).filter(|&(i, j)| is_ok(i, j));
    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
