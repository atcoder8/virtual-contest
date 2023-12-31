use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let mut red = 0;
    let mut blue = 0;
    for c in s.chars() {
        if c == 'R' {
            red += 1;
        } else {
            blue += 1;
        }
    }

    println!("{}", if red > blue { "Yes" } else { "No" });
}
