use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let ans = f(f(f(t) + t) + f(f(t)));
    println!("{}", ans);
}

fn f(x: usize) -> usize {
    x.pow(2) + 2 * x + 3
}
