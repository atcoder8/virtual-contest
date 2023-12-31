use proconio::input;

fn main() {
    input! {
        (a, b): (i64, i64),
    }

    let ans = match a.abs().cmp(&b.abs()) {
        std::cmp::Ordering::Less => "Ant",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Bug",
    };
    println!("{}", ans);
}
