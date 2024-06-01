use itertools::{iproduct, Itertools};
use proconio::input;

const ABILITY_NUM: usize = 5;
const MEMBER_NUM: usize = 3;

fn main() {
    input! {
        n: usize,
        abilities: [[usize; ABILITY_NUM]; n],
    }

    let is_ok = |low_lim: usize| {
        let bits_per_cand = abilities
            .iter()
            .map(|abilities| {
                (0..ABILITY_NUM)
                    .map(|i| ((abilities[i] >= low_lim) as usize) << i)
                    .sum::<usize>()
            })
            .collect_vec();

        let mut dp = [[false; 1 << ABILITY_NUM]; MEMBER_NUM + 1];
        dp[0][0] = true;
        for &bits in &bits_per_cand {
            for (member_cnt, from) in iproduct!((0..MEMBER_NUM).rev(), (0..1 << ABILITY_NUM).rev())
            {
                dp[member_cnt + 1][from | bits] |= dp[member_cnt][from];
            }
        }

        dp[MEMBER_NUM][(1 << ABILITY_NUM) - 1]
    };

    let mut ok = 0_usize;
    let mut ng = 10_usize.pow(9) + 1;

    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
