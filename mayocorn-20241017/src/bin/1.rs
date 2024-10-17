use proconio::{input, marker::Chars};

fn main() {
    input! {
        (mut s, k): (Chars, usize),
    }

    s.sort_unstable();
    for _ in 0..k - 1 {
        next_permutation(&mut s);
    }

    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}

/// If there is a next permutation of `seq` with respect to the lexicographic order, replace `seq` with it (return value is `true`).
/// Otherwise (i.e., if `seq` is already in descending order), it reverts to ascending order (return value is `false`).
pub fn next_permutation<T>(seq: &mut [T]) -> bool
where
    T: Ord,
{
    // If the length of `seq` is 0 or 1, the next permutation does not exist.
    if seq.len() <= 1 {
        return false;
    }

    // Find the maximum value of `i` such that `seq[i] < seq[i + 1]`.
    // If no such `i` exists, `seq` has already been sorted in descending order.
    let Some(i) = (0..seq.len() - 1).rev().find(|&i| seq[i] < seq[i + 1]) else {
        seq.reverse();
        return false;
    };

    // Find the largest `j` that satisfies `i < j` and `seq[i] < seq[j]`, and exchange `seq[i]` and `seq[j]`.
    let j = (i + 1..seq.len()).rev().find(|&j| seq[i] < seq[j]).unwrap();
    seq.swap(i, j);

    // Sort elements after the `i`-th in ascending order to minimize the increase with respect to lexicographic order.
    seq[i + 1..].reverse();

    true
}
