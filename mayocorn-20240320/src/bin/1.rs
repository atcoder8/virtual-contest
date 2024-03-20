use proconio::input;

fn main() {
    input! {
        h: usize,
        a: usize,
    }

    let ans = (h + a - 1) / a;
    println!("{}", ans);
}
