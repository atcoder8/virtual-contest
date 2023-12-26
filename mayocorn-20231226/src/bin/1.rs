use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        ss: [usize; n],
    }

    let ans = ss.iter().filter(|&&s| s <= x).sum::<usize>();
    println!("{}", ans);
}
