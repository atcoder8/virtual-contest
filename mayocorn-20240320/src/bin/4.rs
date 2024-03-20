use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        n: usize,
        (x, y): (f64, f64),
        (opposite_x, opposite_y): (f64, f64),
    }

    let center_x = (x + opposite_x) / 2.0;
    let center_y = (y + opposite_y) / 2.0;

    let radius = (x - center_x).hypot(y - center_y);

    let asin = ((y - center_y) / radius).asin();
    let acos = ((x - center_x) / radius).acos();

    let theta = match (x >= center_x, y >= center_y) {
        (true, true) => asin,
        (true, false) => asin,
        (false, true) => acos,
        (false, false) => 2.0 * PI - acos,
    };
    let next_theta = theta + 2.0 * PI / n as f64;

    let x1 = center_x + radius * next_theta.cos();
    let y1 = center_y + radius * next_theta.sin();
    println!("{} {}", x1, y1);
}
