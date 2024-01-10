use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1..=n).filter(|&i| i % 3 != 0 && i % 5 != 0).sum::<usize>();
    println!("{}", ans);
}
