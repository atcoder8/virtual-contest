use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    let sum = a + b;
    let cycle_num = n / sum;
    let rem = n - sum * cycle_num;
    let blue_num = cycle_num * a + rem.min(a);

    println!("{}", blue_num);
}
