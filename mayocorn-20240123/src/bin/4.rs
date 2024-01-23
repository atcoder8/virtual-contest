use im_rc::HashMap;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (h, w, rs, cs): (usize, usize, Usize1, Usize1),
        n: usize,
        rc: [(Usize1, Usize1); n],
        q: usize,
        dl: [(char, usize); q],
    }

    let mut hor_walls = HashMap::new();
    let mut ver_walls = HashMap::new();
    for &(r, c) in &rc {
        hor_walls.entry(r).or_insert(vec![]).push(c);
        ver_walls.entry(c).or_insert(vec![]).push(r);
    }
    hor_walls.iter_mut().for_each(|x| x.1.sort_unstable());
    ver_walls.iter_mut().for_each(|x| x.1.sort_unstable());

    let find_section = |row: usize, col: usize, horizontal: bool| {
        let max_right = if horizontal { w - 1 } else { h - 1 };

        let Some(walls) = ({
            if horizontal {
                hor_walls.get(&row)
            } else {
                ver_walls.get(&col)
            }
        }) else {
            return (0, max_right)
        };

        let cur_coord = if horizontal { col } else { row };
        let right_wall_pos = walls.upper_bound(&cur_coord);

        let left = walls
            .get(right_wall_pos.wrapping_sub(1))
            .map_or(0, |&left_wall| left_wall + 1);

        let right = walls
            .get(right_wall_pos)
            .map_or(max_right, |&right_wall| right_wall - 1);

        (left, right)
    };

    let mut row = rs;
    let mut col = cs;
    for &(d, l) in &dl {
        let (left, right) = find_section(row, col, true);
        let (top, bottom) = find_section(row, col, false);

        match d {
            'L' => col = col.saturating_sub(l).max(left),
            'R' => col = (col + l).min(right),
            'U' => row = row.saturating_sub(l).max(top),
            'D' => row = (row + l).min(bottom),
            _ => unreachable!(),
        }

        println!("{} {}", row + 1, col + 1);
    }
}
