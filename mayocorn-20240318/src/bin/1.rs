use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans = 0;
    let mut zero = false;
    for c in s.chars() {
        if c == '0' {
            if !zero {
                ans += 1;
            }

            zero = !zero;
        } else {
            ans += 1;
            zero = false;
        }
    }

    println!("{}", ans);
}
