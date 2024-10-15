use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut coords: [(i64, i64); 2],
    }

    for i in 0..2 {
        let (x1, y1) = coords[i];
        let (x2, y2) = coords[i + 1];
        let diff_x = x2 - x1;
        let diff_y = y2 - y1;
        let next_coord = (x2 - diff_y, y2 + diff_x);
        coords.push(next_coord);
    }

    let ans = coords[2..]
        .iter()
        .map(|&(x, y)| format!("{} {}", x, y))
        .join(" ");
    println!("{}", ans);
}
