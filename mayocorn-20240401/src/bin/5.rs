use itertools::Itertools;
use proconio::input;

fn main() {
    match solve() {
        Some(edges) => println!(
            "{}\n{}",
            edges.len(),
            edges
                .iter()
                .map(|&(u, v)| format!("{} {}", u + 1, v + 1))
                .join("\n")
        ),
        None => println!("-1"),
    }
}

fn solve() -> Option<Vec<(usize, usize)>> {
    input! {
        (n, k): (usize, usize),
    }

    if k > (n - 1) * (n - 2) / 2 {
        return None;
    }

    let mut edges = (1..n).map(|u| (0_usize, u)).collect_vec();
    let mut rem = (n - 1) * (n - 2) / 2 - k;
    for i in 1..n {
        if rem == 0 {
            break;
        }

        for j in i + 1..n {
            if rem == 0 {
                break;
            }

            edges.push((i, j));
            rem -= 1;
        }
    }

    Some(edges)
}
