use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = (0..=s.len() - 2)
        .rev()
        .step_by(2)
        .find(|&len| s[..len / 2] == s[len / 2..len])
        .unwrap();
    println!("{}", ans);
}
