use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "correct" } else { "incorrect" });
}

fn solve() -> bool {
    input! {
        n: usize,
        aaa: [Chars; n],
    }

    for i in 0..n {
        for j in i + 1..n {
            match aaa[i][j] {
                'W' if aaa[j][i] != 'L' => return false,
                'L' if aaa[j][i] != 'W' => return false,
                'D' if aaa[j][i] != 'D' => return false,
                _ => {}
            }
        }
    }

    true
}
