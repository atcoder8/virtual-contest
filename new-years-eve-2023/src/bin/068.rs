use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        positions: [usize; 4],
    }

    for &pos in positions.iter().rev() {
        s.insert(pos, '"');
    }

    println!("{}", s.iter().collect::<String>());
}
