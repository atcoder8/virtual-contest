use proconio::input;

fn main() {
    input! {
        (h, n): (usize, usize),
        aa: [usize; n],
    }

    let ans = aa.iter().sum::<usize>() >= h;
    println!("{}", if ans { "Yes" } else { "No" });
}
