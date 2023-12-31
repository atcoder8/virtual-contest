use proconio::input;

fn main() {
    input! {
        day: String,
    }

    let ans = match &day[..] {
        "Sunday" => 0,
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        "Friday" => 1,
        "Saturday" => 0,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
