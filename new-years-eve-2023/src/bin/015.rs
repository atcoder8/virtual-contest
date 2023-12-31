use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let ans = (1_usize..).find(|&n| n.pow(4) == x).unwrap();
    println!("{}", ans);
}
