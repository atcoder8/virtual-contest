use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, t): (usize, usize),
        aa: [usize; n],
    }

    let first_half = aa[..(n / 2)].to_owned();
    let second_half = aa[(n / 2)..].to_owned();

    let first_half_times: Vec<usize> = (0..(1_usize << first_half.len()))
        .map(|bit| {
            (0..first_half.len())
                .filter(|&i| (bit >> i) & 1 == 1)
                .map(|i| first_half[i])
                .sum()
        })
        .collect();

    let mut second_half_times: Vec<usize> = (0..(1_usize << second_half.len()))
        .map(|bit| {
            (0..second_half.len())
                .filter(|&i| (bit >> i) & 1 == 1)
                .map(|i| second_half[i])
                .sum()
        })
        .collect();
    second_half_times.sort_unstable();

    let mut ans = 0;
    for &first_half_time in &first_half_times {
        if first_half_time > t {
            continue;
        }

        let sum_time = first_half_time
            + second_half_times[second_half_times.upper_bound(&(t - first_half_time)) - 1];
        ans = ans.max(sum_time);
    }

    println!("{}", ans);
}
