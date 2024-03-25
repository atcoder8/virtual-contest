use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, k): (usize, usize),
        scores_per_student: [[usize; 3]; n],
    }

    let sum_scores = scores_per_student
        .iter()
        .map(|scores| scores.iter().sum::<usize>())
        .collect_vec();
    let mut sorted = sum_scores.clone();
    sorted.sort_unstable();
    for &sum_score in &sum_scores {
        let pos = sorted.upper_bound(&(sum_score + 300));
        println!("{}", if n - pos < k { "Yes" } else { "No" });
    }
}
