use proconio::input;

fn main() {
    input! {
        (a, p): (usize, usize),
    }

    let ans = (3 * a + p) / 2;
    println!("{}", ans);
}
