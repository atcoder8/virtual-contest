use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    if a + b < 10 {
        println!("{}", a + b);
    } else {
        println!("error");
    }
}
