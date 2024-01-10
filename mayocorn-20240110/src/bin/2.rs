use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, k): (usize, usize),
        ppp: [[usize; 3]; n],
    }

    let scores = ppp.iter().map(|pp| pp.iter().sum::<usize>()).collect_vec();
    let sorted_scores = scores.iter().cloned().sorted_unstable().collect_vec();

    for &score in &scores {
        let ans = n - sorted_scores.upper_bound(&(score + 300)) < k;
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
