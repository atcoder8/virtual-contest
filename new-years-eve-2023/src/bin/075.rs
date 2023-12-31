use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let ans = n.contains('7');
    println!("{}", if ans { "Yes" } else { "No" });
}
