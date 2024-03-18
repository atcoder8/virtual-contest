use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut events: Vec<(usize, i64)> = vec![];
    for &(a, b) in &ab {
        events.push((a, 1));
        events.push((a + b, -1));
    }
    events.sort_unstable_by_key(|v| v.0);

    let mut counts = vec![0; n + 1];
    let mut cnt = 0;
    let mut prev_day = 1;
    for (day, group) in &events.iter().group_by(|v| v.0) {
        counts[cnt as usize] += day - prev_day;
        cnt += group.map(|v| v.1).sum::<i64>();
        prev_day = day;
    }

    println!("{}", counts[1..].iter().join(" "));
}
