use proconio::input;

const WEATHERS: [&str; 3] = ["Sunny", "Cloudy", "Rainy"];

fn main() {
    input! {
        s: String,
    }

    let mut iter = WEATHERS.iter().cycle();
    iter.find(|weather| weather == &&s);
    let ans = iter.next().unwrap();
    println!("{}", ans);
}
