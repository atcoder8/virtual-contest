use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let max = h.max(w);
    let ans = (n + max - 1) / max;
    println!("{}", ans);
}
