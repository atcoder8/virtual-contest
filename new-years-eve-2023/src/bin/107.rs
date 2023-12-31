use proconio::input;

fn main() {
    input! {
        (mut health1, attack1): (i64, i64),
        (mut health2, attack2): (i64, i64),
    }

    let ans = loop {
        health2 -= attack1;
        if health2 <= 0 {
            break true;
        }

        health1 -= attack2;
        if health1 <= 0 {
            break false;
        }
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
