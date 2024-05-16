use itertools::izip;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        lights: [[Usize1]; m],
        pp: [usize; m],
    }

    let is_ok = |bits: usize| {
        izip!(&lights, &pp).all(|(switches, &p)| {
            switches
                .iter()
                .filter(|&&switch| bits >> switch & 1 == 1)
                .count()
                % 2
                == p
        })
    };

    let ans = (0..1 << n).filter(|&bits| is_ok(bits)).count();
    println!("{}", ans);
}
