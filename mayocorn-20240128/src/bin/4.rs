use proconio::input;

fn main() {
    input! {
        (a, b, x): (usize, usize, usize),
    }

    let ans = if a == 0 {
        b / x + 1
    } else {
        b / x - (a - 1) / x
    };
    println!("{}", ans);
}
