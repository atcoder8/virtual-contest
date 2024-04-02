use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        pp: [usize; n],
    }

    let ans = pp.iter().position(|&p| p == x).unwrap() + 1;
    println!("{}", ans);
}
