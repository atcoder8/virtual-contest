use proconio::input;

fn main() {
    input! {
        (lc, sc): (String, String),
    }

    let ans = lc.to_lowercase().to_string() == sc;
    println!("{}", if ans { "Yes" } else { "No" });
}
