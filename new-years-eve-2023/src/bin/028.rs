use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let left = s.chars().position(|c| c == 'A').unwrap();
    let right = s.len() - s.chars().rev().position(|c| c == 'Z').unwrap();
    println!("{}", right - left);
}
