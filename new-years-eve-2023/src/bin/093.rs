use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (m, d): (usize, usize),
    }

    let mut ans = 0;
    for (month, day) in iproduct!(1..=m, 1..=d) {
        let (d1, d10) = (day % 10, day / 10);
        ans += (d1 >= 2 && d10 >= 2 && d1 * d10 == month) as usize;
    }
    println!("{}", ans);
}
