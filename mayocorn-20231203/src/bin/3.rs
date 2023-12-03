use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(u8, i64); n],
    }

    let mut health = 0;
    let mut sick = 0;
    for &(x, y) in &xy {
        if x == 0 {
            health = health.max(health + y).max(sick + y);
        } else {
            sick = sick.max(health + y);
        }
    }

    println!("{}", health.max(sick));
}
