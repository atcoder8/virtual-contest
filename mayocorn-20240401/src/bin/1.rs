use proconio::input;

const WEEK: [&str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

fn main() {
    input! {
        s: String,
    }

    let ans = 7 - WEEK.iter().position(|t| t == &s).unwrap();
    println!("{}", ans);
}
