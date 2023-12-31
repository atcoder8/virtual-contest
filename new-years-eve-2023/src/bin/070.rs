use proconio::input;

fn main() {
    input! {
        a: usize,
        s: String,
    }

    if a >= 3200 {
        println!("{}", s);
    } else {
        println!("red");
    }
}
