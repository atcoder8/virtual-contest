use proconio::input;

fn main() {
    input! {
        (mut a, mut b): (f64, f64),
    }

    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    let mut ok = 0.0_f64;
    let mut ng = std::f64::consts::FRAC_PI_6;

    while (ok - ng).abs() > 1e-12 {
        let mid = (ok + ng) / 2.0;

        if b / mid.cos() * (mid + std::f64::consts::FRAC_PI_3).sin() <= a {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let ans = b / ok.cos();
    println!("{}", ans);
}
