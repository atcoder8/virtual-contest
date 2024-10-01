use proconio::input;

fn main() {
    input! {
        (_n, l): (usize, usize),
        s: String,
    }

    let mut count_crash = 0_usize;
    let mut num_tabs = 1_usize;
    for c in s.chars() {
        if c == '+' {
            num_tabs += 1;
            if num_tabs > l {
                count_crash += 1;
                num_tabs = 1;
            }
        } else {
            num_tabs -= 1;
        }
    }

    println!("{}", count_crash);
}
