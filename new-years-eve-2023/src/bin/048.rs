use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let even_num = k / 2;
    let odd_num = (k + 1) / 2;

    println!("{}", even_num * odd_num);
}
