use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xc: [(i64, Usize1); n],
    }

    let mut section_per_color: Vec<Option<(i64, i64)>> = vec![None; n];
    for &(x, c) in &xc {
        match section_per_color.get_mut(c).unwrap() {
            Some(section) => {
                section.0 = section.0.min(x);
                section.1 = section.1.max(x);
            }
            None => section_per_color[c] = Some((x, x)),
        }
    }

    let mut left = 0_i64;
    let mut left_cost = 0_i64;
    let mut right = 0_i64;
    let mut right_cost = 0_i64;
    for color in 0..n {
        let (next_left, next_right) = match section_per_color[color] {
            Some(section) => section,
            None => continue,
        };

        let next_left_cost =
            (left_cost + (left - next_right).abs() + (next_right - next_left).abs())
                .min(right_cost + (right - next_right).abs() + (next_right - next_left).abs());
        let next_right_cost =
            (left_cost + (left - next_left).abs() + (next_left - next_right).abs())
                .min(right_cost + (right - next_left).abs() + (next_left - next_right).abs());

        left = next_left;
        right = next_right;
        left_cost = next_left_cost;
        right_cost = next_right_cost;
    }

    let ans = (left_cost + left.abs()).min(right_cost + right.abs());
    println!("{}", ans);
}
