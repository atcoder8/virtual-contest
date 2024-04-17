use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t).map(|_| solve()).join("\n");
    println!("{}", ans);
}

fn solve() -> usize {
    input! {
        (n, x, k): (usize, usize, usize),
    }

    let height = floor_log2(n);
    let depth = floor_log2(x);

    let count = |up: usize| {
        if depth + k - 2 * up > height {
            return 0;
        }

        if up == k {
            return 1;
        }

        if up == 0 {
            let left = x << k;
            let right = left + (1 << k);

            return right.min(n + 1).saturating_sub(left);
        }

        let root = x >> (up - 1) ^ 1;
        let down = k - (up + 1);
        let left = root << down;
        let right = left + (1 << down);

        right.min(n + 1).saturating_sub(left)
    };

    (0..=k).take_while(|&up| x >> up != 0).map(count).sum()
}

pub fn floor_log2(n: usize) -> usize {
    (0..).find(|&i| n >> i == 1).unwrap()
}
