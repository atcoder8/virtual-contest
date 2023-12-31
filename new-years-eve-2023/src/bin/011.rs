use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let mut max = 0;
    let mut val = 0;
    for c in s.chars() {
        if c == 'I' {
            val += 1;
        } else {
            val -= 1;
        }

        max = max.max(val);
    }

    println!("{}", max);
}
