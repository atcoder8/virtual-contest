use maplit::hashset;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        _n: usize,
        s: String,
    }

    let mut x = 0;
    let mut y = 0;
    let mut visited = hashset! {(0, 0)};
    for c in s.chars() {
        match c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => unreachable!(),
        }

        if visited.contains(&(x, y)) {
            return true;
        }

        visited.insert((x, y));
    }

    false
}
