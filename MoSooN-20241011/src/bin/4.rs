use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut xx: [i64; n],
    }

    let ans = (0..=n - k)
        .map(|left| {
            let min = xx[left];
            let max = xx[left + k - 1];

            if min >= 0 {
                max
            } else if max <= 0 {
                -min
            } else {
                (2 * max - min).min(2 * -min + max)
            }
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
