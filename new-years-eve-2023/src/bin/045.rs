use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ans = aa.iter().map(|a| a.trailing_zeros()).sum::<u32>();
    println!("{}", ans);
}
