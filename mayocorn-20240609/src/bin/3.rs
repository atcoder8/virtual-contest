use proconio::input;

fn main() {
    input! {
        a: usize,
        b: String,
    }

    let scaled_b = b
        .chars()
        .filter(|&c| c != '.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let ans = a * scaled_b / 100;
    println!("{}", ans);
}
