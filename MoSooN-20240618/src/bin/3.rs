use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    let mut parity = false;
    let mut rem = n;
    while rem != 0 {
        if parity {
            rem = rem.saturating_sub(b);
        } else {
            rem = rem.saturating_sub(a);
        }

        parity = !parity;
    }

    println!("{}", if parity { "Ant" } else { "Bug" });
}
