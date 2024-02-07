use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, usize); n],
    }

    let ans = ab
        .iter()
        .map(|&(a, b)| b * (b + 1) / 2 - a * (a + 1) / 2)
        .sum::<usize>();
    println!("{}", ans);
}
