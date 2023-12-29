use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &abc {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = Some(0);
    }
    for bit in 1..1 << n {
        for from in 0..n {
            let Some(sum_dist) = dp[bit][from] else { continue; };

            for &(to, dist) in &graph[from] {
                if bit >> to & 1 == 1 {
                    continue;
                }

                update_score(&mut dp[bit | 1 << to][to], sum_dist + dist);
            }
        }
    }

    let ans = dp
        .iter()
        .flatten()
        .filter_map(|&sum_dist| sum_dist)
        .max()
        .unwrap();
    println!("{}", ans);
}

/// Updates the maximum score.
/// If `score` is `None`, always updated to the candidate score.
///
/// # Arguments
///
/// * `score` - Reference variable for the score to be updated.
/// * `cand_score` - Candidate score to update.
pub fn update_score<T>(score: &mut Option<T>, cand_score: T) -> bool
where
    T: PartialOrd,
{
    if score.as_ref().is_some_and(|score| score >= &cand_score) {
        return false;
    }

    *score = Some(cand_score);

    true
}
