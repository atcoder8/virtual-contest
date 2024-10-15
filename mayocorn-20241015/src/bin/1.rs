use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = if s.as_str() <= "2019/04/30" {
        "Heisei"
    } else {
        "TBD"
    };
    println!("{}", ans);
}
