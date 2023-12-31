use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        vv: [usize; n],
        cc: [usize; n],
    }

    let ans = izip!(vv, cc)
        .map(|(v, c)| v.saturating_sub(c))
        .sum::<usize>();
    println!("{}", ans);
}
