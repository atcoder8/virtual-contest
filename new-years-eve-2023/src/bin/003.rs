use proconio::input;

fn main() {
    input! {
        age: usize,
    }

    if age == 1 {
        println!("Hello World");
    } else {
        input! {
            (a, b): (usize, usize),
        }

        println!("{}", a + b);
    }
}
