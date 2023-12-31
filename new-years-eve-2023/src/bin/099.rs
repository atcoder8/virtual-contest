use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1..=n).filter(|&i| i.to_string().len() % 2 == 1).count();
    println!("{}", ans);
}
