use proconio::input;

fn main() {
    input! {
        (l1, r1): (usize, usize),
        (l2, r2): (usize, usize),
    }

    let ans = (0..100)
        .filter(|&x| l1 <= x && x < r1 && l2 <= x && x < r2)
        .count();
    println!("{}", ans);
}
