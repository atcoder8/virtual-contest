use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    let ans = (0..=x).any(|crane_num| 2 * crane_num + 4 * (x - crane_num) == y);
    println!("{}", if ans { "Yes" } else { "No" });
}
