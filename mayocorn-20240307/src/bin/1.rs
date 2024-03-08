use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let ans = n
        .chars()
        .map(|c| match c {
            '1' => '9',
            '9' => '1',
            _ => c,
        })
        .collect::<String>();
    println!("{}", ans);
}
