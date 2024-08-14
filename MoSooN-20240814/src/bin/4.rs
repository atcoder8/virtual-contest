use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    let digit_num = x.to_string().len();

    let find_ok = |head: i64, diff: i64| {
        let mut val = head;
        let mut d = head;
        for _ in 1..digit_num {
            d += diff;
            if d < 0 || d > 9 {
                return None;
            }
            val = 10 * val + d;
        }

        if val >= x {
            Some(val)
        } else {
            None
        }
    };

    let head = x.to_string().chars().next().unwrap().to_digit(10).unwrap() as i64;
    let ans = iproduct!(head..=9, -9..=9)
        .find_map(|(head, diff)| find_ok(head, diff))
        .unwrap();
    println!("{}", ans);
}
