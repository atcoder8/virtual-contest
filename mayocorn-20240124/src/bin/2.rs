use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
        bbb: [Chars; h],
    }

    let is_ok = |hor_shift: usize, ver_shift: usize| {
        for (row, col) in iproduct!(0..h, 0..w) {
            if aaa[(row + hor_shift) % h][(col + ver_shift) % w] != bbb[row][col] {
                return false;
            }
        }

        true
    };

    let ans = iproduct!(0..h, 0..w).any(|(hor_shift, ver_shift)| is_ok(hor_shift, ver_shift));
    println!("{}", if ans { "Yes" } else { "No" });
}
