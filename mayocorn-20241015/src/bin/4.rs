use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let events = ab
        .iter()
        .flat_map(|&(a, b)| [(a, true), (a + b, false)])
        .sorted_unstable();
    let mut counts = vec![0_usize; n + 1];
    let mut num_login_users = 0_usize;
    let mut prev_day = 0_usize;
    for (day, group) in &events.group_by(|event| event.0) {
        counts[num_login_users] += day - prev_day;

        for (_, login) in group {
            if login {
                num_login_users += 1;
            } else {
                num_login_users -= 1;
            }
        }

        prev_day = day;
    }

    let ans = counts[1..].iter().join(" ");
    println!("{}", ans);
}
