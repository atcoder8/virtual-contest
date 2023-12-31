use itertools::Itertools;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Heisei" } else { "TBD" });
}

fn solve() -> bool {
    input! {
        s: String,
    }

    let (year, month, day) = s
        .split('/')
        .map(|sub| sub.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    match year.cmp(&2019) {
        std::cmp::Ordering::Less => return true,
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Greater => return false,
    }

    match month.cmp(&4) {
        std::cmp::Ordering::Less => return true,
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Greater => return false,
    }

    day <= 30
}
