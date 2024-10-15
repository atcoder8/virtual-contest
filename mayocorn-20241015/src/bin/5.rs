use itertools::Itertools;
use num::Integer;
use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        pt: [(usize, usize); n - 1],
        q: usize,
        leave_times: [usize; q],
    }

    let calc_bus_time = |rem: usize| {
        let mut time = rem;
        for &(p, t) in &pt {
            time = (time + p - 1) / p * p;
            time += t;
        }

        time - rem
    };

    let lcm = (1..=8).fold(1_usize, |acc, x| acc.lcm(&x));

    let bus_time_per_start = (0..=lcm).map(calc_bus_time).collect_vec();

    let solve = |leave_time: usize| {
        let start = (leave_time + x) % lcm;
        leave_time + x + bus_time_per_start[start] + y
    };

    let ans = leave_times.into_iter().map(solve).join("\n");
    println!("{}", ans);
}
