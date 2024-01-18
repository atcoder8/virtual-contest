use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
    }

    let mut ans = None;
    for a in (1..=n).take_while(|&a| (a - 1).pow(2) <= m) {
        let b = (m + a - 1) / a;
        if b <= n {
            update_cost(&mut ans, a * b);
        }
    }

    ans
}

/// Updates the minimum cost.
/// If `cost` is `None`, always updated to the candidate cost.
///
/// # Arguments
///
/// * `cost` - Reference variable for the cost to be updated.
/// * `cand_cost` - Candidate cost to update.
pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: PartialOrd,
{
    if cost.as_ref().is_some_and(|cost| cost <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
