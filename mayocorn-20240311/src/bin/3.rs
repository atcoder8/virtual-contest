use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut chars = vec![];
    let mut t = n;
    while t != 0 {
        t -= 1;
        chars.push(b'a' + (t % 26) as u8);
        t /= 26;
    }
    chars.reverse();

    let ans = String::from_utf8(chars).unwrap();
    println!("{}", ans);
}
