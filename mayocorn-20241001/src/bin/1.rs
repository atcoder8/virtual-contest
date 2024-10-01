use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xx: [usize; n],
    }

    let ans = 2 * xx.iter().map(|&x| x.min(k - x)).sum::<usize>();
    println!("{}", ans);
}
