use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    let mut stack = vec![(0, 0)];
    while let Some((cur, bit)) = stack.pop() {
        if cur > n {
            continue;
        }

        ans += (bit == 0b111) as usize;

        for (i, add) in enumerate([7, 5, 3]) {
            stack.push((10 * cur + add, bit | 1 << i));
        }
    }

    println!("{}", ans);
}
