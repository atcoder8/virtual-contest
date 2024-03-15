use proconio::input;

fn main() {
    input! {
        (n, t): (usize, usize),
        ct: [(usize, usize); n],
    }

    let ans = ct.iter().filter(|v| v.1 <= t).map(|v| v.0).min();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("TLE"),
    }
}
