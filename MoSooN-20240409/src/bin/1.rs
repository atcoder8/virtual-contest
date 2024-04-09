use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (x, y): (Usize1, Usize1),
    }

    let mut ans = 0;

    if x < 3 {
        ans += 300000 - 100000 * x;
    }

    if y < 3 {
        ans += 300000 - 100000 * y;
    }

    if x == 0 && y == 0 {
        ans += 400000;
    }

    println!("{}", ans);
}
