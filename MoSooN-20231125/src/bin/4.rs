use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut ans: usize = ab.iter().map(|&(a, b)| a.max(b)).sum();
    if ab.iter().filter(|&&(a, b)| a >= b).count() % 2 == 1 {
        ans -= ab.iter().map(|&(a, b)| a.abs_diff(b)).min().unwrap();
    }

    println!("{}", ans);
}
