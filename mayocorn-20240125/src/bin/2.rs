use std::f64::consts::TAU;

use proconio::input;

fn main() {
    input! {
        (a, b, h, m): (f64, f64, f64, f64),
    }

    let rad1 = (60.0 * h + m) / 720.0 * TAU;
    let (sin1, cos1) = rad1.sin_cos();
    let (x1, y1) = (a * sin1, a * cos1);

    let rad2 = m / 60.0 * TAU;
    let (sin2, cos2) = rad2.sin_cos();
    let (x2, y2) = (b * sin2, b * cos2);

    let ans = (x1 - x2).hypot(y1 - y2);
    println!("{}", ans);
}
