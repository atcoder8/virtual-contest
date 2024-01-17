use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut counts = vec![vec![0_usize; 10]; 10];
    for i in 1..=n {
        let head = i.to_string().chars().next().unwrap().to_digit(10).unwrap() as usize;
        let tail = i % 10;

        counts[head][tail] += 1;
    }

    let ans = iproduct!(1..10, 1..10)
        .map(|(head, tail)| counts[head][tail] * counts[tail][head])
        .sum::<usize>();
    println!("{}", ans);
}
