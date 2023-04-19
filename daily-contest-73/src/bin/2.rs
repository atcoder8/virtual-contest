use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let char_to_usize = |c: char| -> usize {
        match c {
            'J' => 0,
            'O' => 1,
            'I' => 2,
            _ => panic!(),
        }
    };

    let t = s.iter().map(|&c| char_to_usize(c)).collect_vec();

    let mut dp = vec![vec![0_usize; 3]];
    for &c in &t {
        let mut next = dp.last().unwrap().clone();
        if c == 0 {
            next[0] += 1;
        } else {
            next[c] += next[c - 1];
        }
        dp.push(next);
    }

    let j_comb_num = {
        let mut dp = vec![vec![0_usize; 3]];
        dp[0][0] = 1;

        for &c in &t {
            let mut next = dp.last().unwrap().clone();
            if c == 0 {
                next[0] += 1;
            } else {
                next[c] += next[c - 1];
            }
            dp.push(next);
        }

        dp[n][2]
    };

    let o_comb_num = {
        let mut max_add = 0;
        let mut j_num = 0;
        let mut i_num = s.iter().filter(|&&c| c == 'I').count();

        for pos in 0..(n - 1) {
            if s[pos] == 'J' {
                j_num += 1;
            } else if s[pos] == 'I' {
                i_num -= 1;
            }

            max_add = max_add.max(j_num * i_num);
        }

        dp[n][2] + max_add
    };

    let i_comb_num = dp[n][2] + dp[n][1];

    let ans = j_comb_num.max(o_comb_num).max(i_comb_num);
    println!("{}", ans);
}
