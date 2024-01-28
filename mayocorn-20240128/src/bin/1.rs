use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = 21 - (a + b + c);
    println!("{}", ans);
}
