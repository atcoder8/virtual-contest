use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
    }

    let ans = h * (w - 1) + (h - 1) * w;
    println!("{}", ans);
}
