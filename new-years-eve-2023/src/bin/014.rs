use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = (a - 1) * (b - 1);
    println!("{}", ans);
}
