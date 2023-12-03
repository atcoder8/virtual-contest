use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        mm: [usize; n],
    }

    let sum_m: usize = mm.iter().sum();
    let min_m = *mm.iter().min().unwrap();

    let ans = n + (x - sum_m) / min_m;
    println!("{}", ans);
}
