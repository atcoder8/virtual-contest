use std::collections::VecDeque;

use im_rc::{HashMap, HashSet};
use proconio::input;
use superslice::Ext;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (_h, _w, n): (usize, usize, usize),
        start_coord: (usize, usize),
        goal_coord: (usize, usize),
        xy: [(usize, usize); n],
    }

    let mut horizontal_walls = HashMap::<usize, Vec<usize>>::new();
    let mut vertical_walls = HashMap::<usize, Vec<usize>>::new();
    for &(x, y) in &xy {
        horizontal_walls.entry(x).or_default().push(y);
        vertical_walls.entry(y).or_default().push(x);
    }
    horizontal_walls
        .iter_mut()
        .for_each(|(_, walls)| walls.sort_unstable());
    vertical_walls
        .iter_mut()
        .for_each(|(_, walls)| walls.sort_unstable());

    let mut visited = HashSet::<(usize, usize)>::new();
    let mut queue = VecDeque::from([(start_coord, 0_usize)]);
    while let Some((coord, cost)) = queue.pop_front() {
        if visited.contains(&coord) {
            continue;
        }

        visited.insert(coord);

        if coord == goal_coord {
            return Some(cost);
        }

        let (x, y) = coord;
        {
            if let Some(walls) = horizontal_walls.get(&x) {
                let row_pos = walls.upper_bound(&y);
                if row_pos > 0 {
                    queue.push_back(((x, walls[row_pos - 1] + 1), cost + 1));
                }
                if row_pos < walls.len() {
                    queue.push_back(((x, walls[row_pos] - 1), cost + 1));
                }
            }
        }
        {
            if let Some(walls) = vertical_walls.get(&y) {
                let col_pos = walls.upper_bound(&x);
                if col_pos > 0 {
                    queue.push_back(((walls[col_pos - 1] + 1, y), cost + 1));
                }
                if col_pos < walls.len() {
                    queue.push_back(((walls[col_pos] - 1, y), cost + 1));
                }
            }
        }
    }

    None
}
