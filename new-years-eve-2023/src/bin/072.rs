use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let rem = n % k;
    let ans = rem.min(k - rem);
    println!("{}", ans);
}
