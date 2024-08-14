use proconio::input;

fn main() {
    input! {
        ss: [String; 3],
        t: String,
    }

    let mut ans = String::new();
    for ch in t.chars() {
        let idx = ch.to_digit(10).unwrap() as usize - 1;
        ans.push_str(&ss[idx]);
    }

    println!("{}", ans);
}
