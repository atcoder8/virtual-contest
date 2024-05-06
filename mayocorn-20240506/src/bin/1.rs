use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let topping = s.chars().filter(|&c| c == 'o').count();
    println!("{}", 700 + 100 * topping);
}
