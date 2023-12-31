use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let is_ok = |x: usize| x * 8 / 100 == a && x * 10 / 100 == b;

    let ans = (1..=20 * b).find(|&x| is_ok(x));
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
