use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let max_a = ab.iter().map(|x| x.0).max().unwrap();
    let min_b = ab.iter().map(|x| x.1).min().unwrap();
    println!("{}", max_a + min_b);
}
