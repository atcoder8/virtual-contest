use num_integer::Integer;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let mut half_lcm = 1;
    for &a in &aa {
        half_lcm = half_lcm.lcm(&(a / 2));

        if half_lcm > m {
            return 0;
        }
    }

    if aa.iter().any(|&a| (half_lcm - a / 2) % a != 0) {
        return 0;
    }

    (m - half_lcm) / (2 * half_lcm) + 1
}
