use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        corners: [(i64, i64); 4],
    }

    let ans = corners
        .iter()
        .circular_tuple_windows()
        .all(|(coord1, coord2, coord3)| {
            let dx1 = coord2.0 - coord1.0;
            let dy1 = coord2.1 - coord1.1;

            let dx2 = coord3.0 - coord1.0;
            let dy2 = coord3.1 - coord1.1;

            cross_prod((dx1, dy1), (dx2, dy2)) > 0
        });
    println!("{}", if ans { "Yes" } else { "No" });
}

fn cross_prod(v1: (i64, i64), v2: (i64, i64)) -> i64 {
    v1.0 * v2.1 - v2.0 * v1.1
}
