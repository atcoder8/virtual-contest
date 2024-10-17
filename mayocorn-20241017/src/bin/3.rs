use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        coords: [(i64, i64); 4],
    }

    let ans = coords
        .iter()
        .circular_tuple_windows()
        .all(|(&coord1, &coord2, &coord3)| cross_product(coord1, coord2, coord3) > 0);
    println!("{}", if ans { "Yes" } else { "No" });
}

fn cross_product(coord1: (i64, i64), coord2: (i64, i64), coord3: (i64, i64)) -> i64 {
    let (dx1, dy1) = (coord2.0 - coord1.0, coord2.1 - coord1.1);
    let (dx2, dy2) = (coord3.0 - coord2.0, coord3.1 - coord2.1);
    dx1 * dy2 - dx2 * dy1
}
