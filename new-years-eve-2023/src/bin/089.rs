use itertools::izip;
use proconio::input;

fn main() {
    input! {
        (n, m, c): (usize, usize, i64),
        bb: [i64; m],
        aaa: [[i64; m]; n],
    }

    let is_ok = |aa: &[i64]| izip!(aa, &bb).map(|(a, b)| a * b).sum::<i64>() + c > 0;

    let ans = aaa.iter().filter(|aa| is_ok(aa)).count();
    println!("{}", ans);
}
