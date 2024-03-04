use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        xx: [usize; n],
        cy: [(usize, usize); m],
    }

    let mut bonuses = vec![0; n + 1];
    for &(c, y) in &cy {
        bonuses[c] += y;
    }

    let mut dp: Vec<Option<usize>> = vec![Some(0)];
    for &x in &xx {
        let tail = dp.iter().filter_map(|&score| score).max().unwrap();
        dp.insert(0, Some(tail));

        for (i, score) in enumerate(dp[1..].iter_mut()) {
            if let Some(score) = score {
                *score += x + bonuses[i + 1];
            }
        }
    }

    let ans = dp.iter().filter_map(|&score| score).max().unwrap();
    println!("{}", ans);
}
