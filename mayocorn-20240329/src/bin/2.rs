use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let is_ok = |mut coords: Vec<(i64, i64)>| {
        coords.sort_unstable_by_key(|v| v.0);

        if coords[0].0 == coords[1].0 {
            return coords[0].0 == coords[2].0;
        }

        let dx1 = coords[0].0 - coords[1].0;
        let dy1 = coords[0].1 - coords[1].1;

        let dx2 = coords[0].0 - coords[2].0;
        let dy2 = coords[0].1 - coords[2].1;

        dx1 * dy2 == dx2 * dy1
    };

    let ans = xy.into_iter().combinations(3).any(|coords| is_ok(coords));
    println!("{}", if ans { "Yes" } else { "No" });
}
