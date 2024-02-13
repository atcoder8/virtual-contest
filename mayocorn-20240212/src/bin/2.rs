use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some((row, col)) => println!("{} {}", row + 1, col + 1),
        None => println!("-1"),
    }
}

fn solve() -> Option<(usize, usize)> {
    input! {
        (h, w): (usize, usize),
        grid: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];
    let mut row = 0;
    let mut col = 0;
    loop {
        match grid[row][col] {
            'U' => {
                if row == 0 {
                    return Some((row, col));
                }

                row -= 1;
            }
            'D' => {
                if row == h - 1 {
                    return Some((row, col));
                }

                row += 1;
            }
            'L' => {
                if col == 0 {
                    return Some((row, col));
                }

                col -= 1;
            }
            'R' => {
                if col == w - 1 {
                    return Some((row, col));
                }

                col += 1;
            }
            _ => panic!(),
        }

        if visited[row][col] {
            return None;
        }

        visited[row][col] = true;
    }
}
