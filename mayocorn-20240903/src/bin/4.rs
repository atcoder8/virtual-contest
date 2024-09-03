use std::collections::VecDeque;

use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (a, n): (usize, usize),
    }

    let num_digits = n.to_string().len();
    let max = 10_usize.pow(num_digits as u32);
    let mut visited = FixedBitSet::with_capacity(max + 1);
    let mut queue = VecDeque::from([(1_usize, 0_usize)]);
    while let Some((x, cost)) = queue.pop_front() {
        if visited[x] {
            continue;
        }

        visited.insert(x);

        if x == n {
            return Some(cost);
        }

        if a * x <= max {
            queue.push_back((a * x, cost + 1));
        }
        if x % 10 != 0 {
            let mut str_x = x.to_string();
            let c = str_x.pop().unwrap();
            str_x.insert(0, c);
            let next_x = str_x.parse::<usize>().unwrap();
            queue.push_back((next_x, cost + 1));
        }
    }

    None
}
