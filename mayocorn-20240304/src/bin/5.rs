use itertools::{enumerate, Itertools};
use proconio::input;

const MOD: usize = 200;

fn main() {
    match solve() {
        Some((bb, cc)) => println!(
            "Yes\n{} {}\n{} {}",
            bb.len(),
            bb.iter().map(|b| b + 1).join(" "),
            cc.len(),
            cc.iter().map(|c| c + 1).join(" ")
        ),
        None => println!("No"),
    }
}

fn solve() -> Option<(Vec<usize>, Vec<usize>)> {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.iter_mut().for_each(|a| *a %= MOD);

    let restore = |dp: &[Option<usize>], start_rem: usize| {
        let mut seq = vec![];
        let mut rem = start_rem;
        while rem != 0 {
            let idx = dp[rem].unwrap();
            seq.push(idx);
            rem = (rem + MOD - aa[idx]) % MOD;
        }
        seq.reverse();

        seq
    };

    let mut dp: Vec<Option<usize>> = vec![None; MOD];
    for (i, &a) in enumerate(&aa) {
        if dp[0].is_some() {
            let mut bb = restore(&dp, (MOD - aa[dp[0].unwrap()]) % MOD);
            bb.extend([dp[0].unwrap(), i]);
            let cc = vec![i];

            return Some((bb, cc));
        }

        if dp[a].is_some() {
            return Some((restore(&dp, a), vec![i]));
        }

        let mut next_dp = dp.clone();
        next_dp[a] = Some(i);

        for rem in 0..MOD {
            if dp[rem].is_none() {
                continue;
            }

            let next_rem = (rem + a) % MOD;

            if dp[next_rem].is_some() {
                let bb = restore(&dp, next_rem);
                let mut cc = restore(&dp, rem);
                cc.push(i);

                return Some((bb, cc));
            }

            next_dp[next_rem] = Some(i);
        }

        dp = next_dp;
    }

    None
}
