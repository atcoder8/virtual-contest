use std::f64::consts::PI;

use itertools::Itertools;
use proconio::input;

type Coord = (f64, f64);

const EPS: f64 = 1e-8;
const ALLOWED_ERROR: f64 = 1e-7;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let coords = xy.iter().map(|&(x, y)| (x as f64, y as f64)).collect_vec();

    let is_ok = |radius: f64| {
        let mut intersections = vec![];
        for coord_pair in coords.iter().combinations(2) {
            let (coord1, coord2) = (*coord_pair[0], *coord_pair[1]);
            let dist = calc_dist(coord1, coord2);
            let sq_h = radius.powi(2) - (dist / 2.0).powi(2);

            if sq_h < 0.0 {
                continue;
            }

            let h = sq_h.sqrt();
            let center = ((coord1.0 + coord2.0) / 2.0, (coord1.1 + coord2.1) / 2.0);
            let theta = (coord2.1 - coord1.1).atan2(coord2.0 - coord1.0);

            intersections.push((
                center.0 + h * (theta + PI / 2.0).cos(),
                center.1 + h * (theta + PI / 2.0).sin(),
            ));

            intersections.push((
                center.0 + h * (theta - PI / 2.0).cos(),
                center.1 + h * (theta - PI / 2.0).sin(),
            ));
        }

        intersections.iter().any(|&intersection| {
            coords
                .iter()
                .all(|&coord| calc_dist(intersection, coord) <= radius + EPS)
        })
    };

    let mut ok = 1e6_f64;
    let mut ng = 0.0;
    while (ok - ng).abs() > ALLOWED_ERROR {
        let mid = (ok + ng) / 2.0;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn calc_dist(coord1: Coord, coord2: Coord) -> f64 {
    ((coord1.0 - coord2.0).powi(2) + (coord1.1 - coord2.1).powi(2)).sqrt()
}
