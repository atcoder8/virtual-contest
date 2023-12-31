use proconio::input;

fn main() {
    input! {
        (a, b): (String, String),
    }

    let c = (a + &b).parse::<usize>().unwrap();
    let ans = (0_usize..)
        .take_while(|i| i.pow(2) <= c)
        .any(|i| i.pow(2) == c);
    println!("{}", if ans { "Yes" } else { "No" });
}
