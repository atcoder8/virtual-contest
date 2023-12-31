use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let ans = a.min(b) + c.min(d);
    println!("{}", ans);
}
