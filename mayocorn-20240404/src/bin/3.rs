use proconio::input;

fn main() {
    input! {
        (a, b, x): (f64, f64, f64),
    }

    let is_ok = |rad: f64| {
        let tan = rad.tan();

        if a * tan >= b {
            a * b.powi(2) / (2.0 * tan) >= x
        } else {
            a.powi(2) * (a * tan / 2.0 + (b - a * tan)) >= x
        }
    };

    let mut ok = 0.0;
    let mut ng = std::f64::consts::PI / 2.0;
    while (ok - ng).abs() > 1e-9 {
        let mid = (ok + ng) / 2.0;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok.to_degrees());
}
