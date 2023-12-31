use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = match n {
        0..=59 => "Bad",
        60..=89 => "Good",
        90..=99 => "Great",
        100 => "Perfect",
        _ => unreachable!()
    };
    println!("{}", ans);
}
