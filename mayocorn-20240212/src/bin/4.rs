use std::collections::BTreeMap;

use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    if n == 1 {
        return calc_sum(aa[0], k);
    }

    let mut counter: BTreeMap<usize, usize> = BTreeMap::new();
    for &a in &aa {
        *counter.entry(a).or_default() += 1;
    }
    counter.insert(0, 0);

    let mut ans = 0;
    let mut rem = k;
    loop {
        let (a1, cnt1) = counter.pop_last().unwrap();

        if a1 == 0 {
            break;
        }

        let (a2, cnt2) = counter.pop_last().unwrap();

        let diff = a1 - a2;

        if rem <= diff * cnt1 {
            let (q, r) = (rem / cnt1, rem % cnt1);
            ans += calc_sum(a1, q) * cnt1 + (a1 - q) * r;

            break;
        }

        ans += calc_sum(a1, diff) * cnt1;
        rem -= diff * cnt1;
        counter.insert(a2, cnt1 + cnt2);
    }

    ans
}

fn calc_sum(start: usize, len: usize) -> usize {
    let diff = start.saturating_sub(len);

    (start * (start + 1) - diff * (diff + 1)) / 2
}
