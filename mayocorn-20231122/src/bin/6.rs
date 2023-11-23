// unfinished

use itertools::Itertools;
use proconio::input;

type Coord = (f64, f64);

const EPS: f64 = 1e-8;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let coords = xy.iter().map(|&(x, y)| (x as f64, y as f64)).collect_vec();

    let mut ans = f64::INFINITY;
    for comb in (0..n).combinations(2) {
        let coord1 = convert_to_f64(xy[comb[0]]);
        let coord2 = convert_to_f64(xy[comb[1]]);

        let center = calc_center(coord1, coord2);
        let radius = calc_dist(center, coord1);

        if coords
            .iter()
            .all(|&coord| calc_dist(center, coord) <= radius + EPS)
            && radius < ans
        {
            ans = radius;
        }
    }

    for comb in (0..n).combinations(3) {
        let comb = comb
            .iter()
            .sorted_unstable_by_key(|&&i| xy[i].0)
            .cloned()
            .collect_vec();
        let coord0 = xy[comb[0]];
        let coord1 = xy[comb[1]];
        let coord2 = xy[comb[2]];

        if ((coord0.0 - coord1.0) * (coord0.1 - coord2.1)
            - (coord0.0 - coord2.0) * (coord0.1 - coord1.1))
            == 0
        {
            continue;
        }

        let coord0 = coords[comb[0]];
        let coord1 = coords[comb[1]];
        let coord2 = coords[comb[2]];

        let a = calc_dist(coord0, coord1);
        let b = calc_dist(coord1, coord2);
        let c = calc_dist(coord2, coord0);

        let corner_a = calc_corner_radian(a, b, c);
        let corner_b = calc_corner_radian(b, c, a);
        let corner_c = calc_corner_radian(c, a, b);

        let radius = a / (2.0 * corner_a.sin());

        let sin2a = (2.0 * corner_a).sin();
        let sin2b = (2.0 * corner_b).sin();
        let sin2c = (2.0 * corner_c).sin();

        let denom = sin2a + sin2b + sin2c;

        let center_x = (sin2a * coord0.0 + sin2b * coord1.0 + sin2c * coord2.0) / denom;
        let center_y = (sin2a * coord0.1 + sin2b * coord1.1 + sin2c * coord2.1) / denom;
        let center = (center_x, center_y);

        if coords
            .iter()
            .all(|&coord| calc_dist(center, coord) <= radius + EPS)
            && radius < ans
        {
            ans = radius;
        }
    }

    println!("{}", ans);
}

fn convert_to_f64(coord: (i64, i64)) -> Coord {
    (coord.0 as f64, coord.1 as f64)
}

fn calc_center(coord1: Coord, coord2: Coord) -> Coord {
    ((coord1.0 + coord2.0) / 2.0, (coord1.1 + coord2.1) / 2.0)
}

fn calc_dist(coord1: Coord, coord2: Coord) -> f64 {
    ((coord1.0 - coord2.0).powi(2) + (coord1.1 - coord2.1).powi(2)).sqrt()
}

fn calc_corner_radian(a: f64, b: f64, c: f64) -> f64 {
    ((b.powi(2) + c.powi(2) - a.powi(2)) / (2.0 * b * c)).acos()
}
