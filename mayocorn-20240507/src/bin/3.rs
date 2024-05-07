use itertools::{izip, Itertools};
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        s: String,
    }

    enum Dir {
        Left,
        Right,
    }

    struct Person {
        x: usize,
        y: usize,
        dir: Dir,
    }

    let mut persons = izip!(&xy, s.chars())
        .map(|(&(x, y), c)| Person {
            x,
            y,
            dir: if c == 'L' { Dir::Left } else { Dir::Right },
        })
        .sorted_unstable_by_key(|person| person.y)
        .collect_vec();

    let mut left = 0;
    while left < n {
        let right = left + persons[left..].partition_point(|person| person.y == persons[left].y);
        persons[left..right].sort_unstable_by_key(|person| person.x);
        for (person1, person2) in persons[left..right].iter().tuple_windows() {
            if matches!(person1.dir, Dir::Right) && matches!(person2.dir, Dir::Left) {
                return true;
            }
        }

        left = right;
    }

    false
}
