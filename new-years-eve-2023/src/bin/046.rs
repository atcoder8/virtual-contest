use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        tt: [usize; n],
    }

    let ans = (2..n).find(|&day| tt[day - 2] + tt[day - 1] + tt[day] < k);
    match ans {
        Some(ans) => println!("{}", ans + 1),
        None => println!("-1"),
    }
}
