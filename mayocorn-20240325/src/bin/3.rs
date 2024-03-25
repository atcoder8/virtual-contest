use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ll: [usize; n],
    }

    ll.sort_unstable();

    let mut ans = 0;
    let mut counts = vec![0_usize; 2001];
    for (i, &l) in enumerate(&ll) {
        ans += counts[l + 1..].iter().sum::<usize>();

        for &other_l in &ll[0..i] {
            counts[l + other_l] += 1;
        }
    }

    println!("{}", ans);
}
