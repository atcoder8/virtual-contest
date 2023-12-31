use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let hours = n / 3600;
    let minutes = n % 3600 / 60;
    let seconds = n % 60;

    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}
